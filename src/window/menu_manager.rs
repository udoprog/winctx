use std::mem::MaybeUninit;
use std::ptr;

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::MSG;

use super::WindowEvent;

pub(super) const ICON_MSG_ID: u32 = winuser::WM_USER + 1;

/// Helper to manager clipboard polling state.
pub(super) struct MenuManager<'a> {
    events_tx: &'a UnboundedSender<WindowEvent>,
    hmenu: winuser::HMENU,
}

impl<'a> MenuManager<'a> {
    pub(super) fn new(events_tx: &'a UnboundedSender<WindowEvent>, hmenu: winuser::HMENU) -> Self {
        Self { events_tx, hmenu }
    }

    pub(super) unsafe fn dispatch(&mut self, msg: &MSG) -> bool {
        match msg.message {
            ICON_MSG_ID => {
                match msg.lParam as u32 {
                    // Balloon clicked.
                    shellapi::NIN_BALLOONUSERCLICK => {
                        _ = self.events_tx.send(WindowEvent::BalloonClicked);
                        return true;
                    }
                    // Balloon timed out.
                    shellapi::NIN_BALLOONTIMEOUT => {
                        _ = self.events_tx.send(WindowEvent::BalloonTimeout);
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
                            self.hmenu,
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
                let menu_id = winuser::GetMenuItemID(self.hmenu, msg.wParam as i32) as i32;

                if menu_id != -1 {
                    _ = self
                        .events_tx
                        .send(WindowEvent::MenuClicked(menu_id as u32));
                }

                return true;
            }
            _ => {}
        }

        false
    }
}
