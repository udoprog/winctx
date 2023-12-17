use std::mem::MaybeUninit;
use std::ptr;

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::{HMENU, MSG};

use crate::AreaId;

use super::messages;
use super::WindowEvent;

/// Helper to manager clipboard polling state.
pub(super) struct MenuManager<'a> {
    events_tx: &'a UnboundedSender<WindowEvent>,
    hmenus: &'a [Option<winuser::HMENU>],
}

impl<'a> MenuManager<'a> {
    pub(super) fn new(
        events_tx: &'a UnboundedSender<WindowEvent>,
        hmenus: &'a [Option<winuser::HMENU>],
    ) -> Self {
        Self { events_tx, hmenus }
    }

    pub(super) unsafe fn dispatch(&mut self, msg: &MSG) -> bool {
        match msg.message {
            messages::ICON_ID => {
                let area_id = AreaId::new(msg.wParam as u32);

                match msg.lParam as u32 {
                    // Balloon clicked.
                    shellapi::NIN_BALLOONUSERCLICK => {
                        _ = self
                            .events_tx
                            .send(WindowEvent::NotificationClicked(area_id));
                        return true;
                    }
                    // Balloon timed out.
                    shellapi::NIN_BALLOONTIMEOUT => {
                        _ = self
                            .events_tx
                            .send(WindowEvent::NotificationDismissed(area_id));
                        return true;
                    }
                    winuser::WM_LBUTTONUP | winuser::WM_RBUTTONUP => {
                        _ = self.events_tx.send(WindowEvent::IconClicked(area_id));

                        let Some(Some(hmenu)) = self.hmenus.get(area_id.id() as usize) else {
                            return true;
                        };

                        let mut p = MaybeUninit::zeroed();

                        if winuser::GetCursorPos(p.as_mut_ptr()) == FALSE {
                            return true;
                        }

                        let p = p.assume_init();

                        winuser::SetForegroundWindow(msg.hwnd);

                        winuser::TrackPopupMenu(
                            *hmenu,
                            0,
                            p.x,
                            p.y,
                            (winuser::TPM_BOTTOMALIGN | winuser::TPM_LEFTALIGN) as i32,
                            msg.hwnd,
                            ptr::null_mut(),
                        );

                        return true;
                    }
                    _ => (),
                }
            }
            winuser::WM_MENUCOMMAND => {
                let hmenu = msg.lParam as HMENU;

                let Some(area_id) = self.hmenus.iter().position(|h| *h == Some(hmenu)) else {
                    return true;
                };

                _ = self.events_tx.send(WindowEvent::MenuItemClicked(
                    AreaId::new(area_id as u32),
                    msg.wParam as u32,
                ));

                return true;
            }
            _ => {}
        }

        false
    }
}
