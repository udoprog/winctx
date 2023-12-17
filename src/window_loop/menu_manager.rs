use std::mem::MaybeUninit;
use std::ptr;

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::MSG;

use super::messages;
use super::WindowEvent;

/// Helper to manager clipboard polling state.
pub(super) struct MenuManager<'a> {
    events_tx: &'a UnboundedSender<WindowEvent>,
    hmenus: &'a [winuser::HMENU],
}

impl<'a> MenuManager<'a> {
    pub(super) fn new(
        events_tx: &'a UnboundedSender<WindowEvent>,
        hmenus: &'a [winuser::HMENU],
    ) -> Self {
        Self { events_tx, hmenus }
    }

    pub(super) unsafe fn dispatch(&mut self, msg: &MSG) -> bool {
        match msg.message {
            messages::ICON_ID => {
                match msg.lParam as u32 {
                    // Balloon clicked.
                    shellapi::NIN_BALLOONUSERCLICK => {
                        _ = self.events_tx.send(WindowEvent::NotificationClicked);
                        return true;
                    }
                    // Balloon timed out.
                    shellapi::NIN_BALLOONTIMEOUT => {
                        _ = self.events_tx.send(WindowEvent::NotificationDismissed);
                        return true;
                    }
                    winuser::WM_LBUTTONUP | winuser::WM_RBUTTONUP => {
                        let mut p = MaybeUninit::zeroed();

                        if winuser::GetCursorPos(p.as_mut_ptr()) == FALSE {
                            return true;
                        }

                        let p = p.assume_init();

                        winuser::SetForegroundWindow(msg.hwnd);

                        winuser::TrackPopupMenu(
                            self.hmenus[0],
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
                let menu_id = winuser::GetMenuItemID(self.hmenus[0], msg.wParam as i32) as i32;

                if menu_id != -1 {
                    _ = self
                        .events_tx
                        .send(WindowEvent::MenuItemClicked(menu_id as u32));
                }

                return true;
            }
            _ => {}
        }

        false
    }
}
