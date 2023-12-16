use std::str;

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::MSG;

use crate::clipboard::{Clipboard, ClipboardFormat};
use crate::error::{ErrorKind, WindowError};
use crate::{ClipboardEvent, Error};

use super::WindowEvent;

const CLIPBOARD_RETRY_TIMER: usize = 1000;
const RETRY_MILLIS: u32 = 25;
const RETRY_MAX_ATTEMPTS: usize = 10;

/// A timer used to debounce reacting to clipboard updates.
///
/// We will only process updates again after this timer has been fired.
const CLIPBOARD_DEBOUNCE_TIMER: usize = 1001;
const DEBOUNCE_MILLIS: u32 = 25;

/// Helper to manager clipboard polling state.
pub(super) struct ClipboardManager<'a> {
    events_tx: &'a UnboundedSender<WindowEvent>,
    attempts: usize,
    supported: Option<ClipboardFormat>,
}

impl<'a> ClipboardManager<'a> {
    pub(super) fn new(events_tx: &'a UnboundedSender<WindowEvent>) -> Self {
        Self {
            events_tx,
            attempts: 0,
            supported: None,
        }
    }

    pub(super) unsafe fn dispatch(&mut self, msg: &MSG) -> bool {
        match msg.message {
            winuser::WM_CLIPBOARDUPDATE => {
                // Debounce incoming events.
                winuser::SetTimer(msg.hwnd, CLIPBOARD_DEBOUNCE_TIMER, DEBOUNCE_MILLIS, None);
                true
            }
            winuser::WM_TIMER => match msg.wParam {
                CLIPBOARD_RETRY_TIMER => {
                    self.handle_timer(msg.hwnd);
                    true
                }
                CLIPBOARD_DEBOUNCE_TIMER => {
                    winuser::KillTimer(msg.hwnd, CLIPBOARD_DEBOUNCE_TIMER);
                    self.populate_formats();

                    // We need to incorporate a little delay to avoid "clobbering"
                    // the clipboard, since it might still be in use by the
                    // application that just updated it. Including the resources
                    // that were apart of the update.
                    //
                    // Note that there are two distinct states we might clobber:
                    // * The clipboard itself may only be open by one process at a
                    //   time.
                    // * Any resources sent over the clipboard may only be locked by
                    //   one process at a time (GlobalLock / GlobalUnlock).
                    //
                    // If these overlap in the sending process, it might result in
                    // it ironically enough failing to send the clipboard data.
                    //
                    // So as a best effort, we impose a minor timeout of
                    // INITIAL_MILLIS to hopefully avoid this.
                    let Ok(result) = self.poll_clipboard(msg.hwnd) else {
                        winuser::SetTimer(msg.hwnd, CLIPBOARD_RETRY_TIMER, RETRY_MILLIS, None);
                        self.attempts = 1;
                        return true;
                    };

                    if let Some(clipboard_event) = result {
                        _ = self.events_tx.send(WindowEvent::Clipboard(clipboard_event));
                    }

                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn populate_formats(&mut self) {
        self.supported = 'out: {
            for format in Clipboard::updated_formats::<16>() {
                if matches!(
                    format,
                    ClipboardFormat::DIBV5 | ClipboardFormat::TEXT | ClipboardFormat::UNICODETEXT
                ) {
                    break 'out Some(format);
                }
            }

            None
        };
    }

    unsafe fn handle_timer(&mut self, hwnd: HWND) {
        let result = match self.poll_clipboard(hwnd) {
            Ok(result) => result,
            Err(error) => {
                if self.attempts >= RETRY_MAX_ATTEMPTS {
                    winuser::KillTimer(hwnd, CLIPBOARD_RETRY_TIMER);
                    self.attempts = 0;
                    _ = self.events_tx.send(WindowEvent::Error(Error::new(
                        ErrorKind::ClipboardPoll(error),
                    )));
                } else {
                    if self.attempts == 0 {
                        winuser::SetTimer(hwnd, CLIPBOARD_RETRY_TIMER, RETRY_MILLIS, None);
                    }

                    self.attempts += 1;
                }

                return;
            }
        };

        winuser::KillTimer(hwnd, CLIPBOARD_RETRY_TIMER);
        self.attempts = 0;

        if let Some(clipboard_event) = result {
            _ = self.events_tx.send(WindowEvent::Clipboard(clipboard_event));
        }
    }

    pub(super) unsafe fn poll_clipboard(
        &mut self,
        hwnd: HWND,
    ) -> Result<Option<ClipboardEvent>, WindowError> {
        let clipboard = Clipboard::new(hwnd).map_err(WindowError::OpenClipboard)?;

        let Some(format) = self.supported else {
            return Ok(None);
        };

        let data = clipboard
            .data(format)
            .map_err(WindowError::GetClipboardData)?;
        let data = data.lock().map_err(WindowError::LockClipboardData)?;

        // We've successfully locked the data, so take it from here.
        self.supported = None;

        let clipboard_event = match format {
            ClipboardFormat::DIBV5 => ClipboardEvent::BitMap(data.as_slice().to_vec()),
            ClipboardFormat::TEXT => {
                let data = data.as_slice();

                let data = match data {
                    [head @ .., 0] => head,
                    rest => rest,
                };

                let Ok(string) = str::from_utf8(data) else {
                    return Ok(None);
                };

                ClipboardEvent::Text(string.to_owned())
            }
            ClipboardFormat::UNICODETEXT => {
                let data = data.as_wide_slice();

                let data = match data {
                    [head @ .., 0] => head,
                    rest => rest,
                };

                let Ok(string) = String::from_utf16(data) else {
                    return Ok(None);
                };

                ClipboardEvent::Text(string.to_owned())
            }
            _ => {
                return Ok(None);
            }
        };

        Ok(Some(clipboard_event))
    }
}
