use std::mem::MaybeUninit;
use std::ptr;

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::{HMENU, MSG};

use crate::event::MouseButton;
use crate::event::MouseButtons;
use crate::event::MouseEvent;
use crate::AreaId;

use super::messages;
use super::WindowEvent;

/// Helper to manager clipboard polling state.
pub(super) struct MenuManager<'a> {
    events_tx: &'a UnboundedSender<WindowEvent>,
    menus: &'a [Option<(winuser::HMENU, MouseButtons)>],
}

impl<'a> MenuManager<'a> {
    pub(super) fn new(
        events_tx: &'a UnboundedSender<WindowEvent>,
        menus: &'a [Option<(winuser::HMENU, MouseButtons)>],
    ) -> Self {
        Self { events_tx, menus }
    }

    pub(super) unsafe fn dispatch(&mut self, msg: &MSG) -> bool {
        match msg.message {
            messages::ICON_ID => {
                let area_id = AreaId::new(msg.wParam as u32);

                match msg.lParam as u32 {
                    // Balloon clicked.
                    shellapi::NIN_BALLOONUSERCLICK => {
                        let event = MouseEvent {
                            buttons: MouseButtons::empty(),
                        };

                        _ = self
                            .events_tx
                            .send(WindowEvent::NotificationClicked(area_id, event));
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
                        let button = match msg.lParam as u32 {
                            winuser::WM_LBUTTONUP => MouseButton::Left,
                            winuser::WM_RBUTTONUP => MouseButton::Right,
                            _ => return true,
                        };

                        _ = self.events_tx.send(WindowEvent::IconClicked(
                            area_id,
                            MouseEvent {
                                buttons: MouseButtons::from_iter([button]),
                            },
                        ));

                        let Some(Some((hmenu, open_menu))) = self.menus.get(area_id.id() as usize)
                        else {
                            return true;
                        };

                        if !open_menu.test(button) {
                            return true;
                        }

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

                let Some(area_id) = self
                    .menus
                    .iter()
                    .position(|el| el.as_ref().map(|(h, _)| *h) == Some(hmenu))
                else {
                    return true;
                };

                let event = MouseEvent {
                    buttons: MouseButtons::empty(),
                };

                _ = self.events_tx.send(WindowEvent::MenuItemClicked(
                    AreaId::new(area_id as u32),
                    msg.wParam as u32,
                    event,
                ));

                return true;
            }
            _ => {}
        }

        false
    }
}
