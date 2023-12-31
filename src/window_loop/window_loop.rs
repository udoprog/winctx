#![allow(clippy::field_reassign_with_default)]

use std::ffi::OsStr;
use std::io;
use std::mem::size_of;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::thread;

use tokio::sync::mpsc;
use tokio::sync::oneshot;
use windows_sys::Win32::Foundation::{FALSE, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::DataExchange::AddClipboardFormatListener;
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

use crate::convert::ToWide;
use crate::error::ErrorKind::*;
use crate::error::{Error, WindowError};
use crate::event::{ClipboardEvent, MouseEvent};
use crate::window_loop::messages;
use crate::AreaId;
use crate::Result;

use super::{AreaHandle, ClipboardManager, MenuManager, WindowClassHandle, WindowHandle};

#[derive(Debug)]
pub(crate) enum WindowEvent {
    /// A meny item was clicked.
    MenuItemClicked(AreaId, u32, MouseEvent),
    /// Shutdown was requested.
    Shutdown,
    /// Clipboard event.
    Clipboard(ClipboardEvent),
    /// The notification icon has been clicked.
    IconClicked(AreaId, MouseEvent),
    /// Balloon was clicked.
    NotificationClicked(AreaId, MouseEvent),
    /// Balloon timed out.
    NotificationDismissed(AreaId),
    /// Data copied to this process.
    CopyData(usize, Vec<u8>),
    /// Non-fatal error.
    Error(Error),
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // Match over all messages we want to post back to the event loop.
    match msg {
        messages::ICON_ID => {
            if matches!(
                l_param as u32,
                shellapi::NIN_BALLOONUSERCLICK
                    | shellapi::NIN_BALLOONTIMEOUT
                    | winuser::WM_LBUTTONUP
                    | winuser::WM_RBUTTONUP
            ) {
                winuser::PostMessageW(hwnd, msg, w_param, l_param);
                return 0;
            }
        }
        winuser::WM_MENUCOMMAND => {
            winuser::PostMessageW(hwnd, msg, w_param, l_param);
            return 0;
        }
        winuser::WM_CLIPBOARDUPDATE => {
            winuser::PostMessageW(hwnd, msg, w_param, l_param);
            return 0;
        }
        winuser::WM_DESTROY => {
            winuser::PostMessageW(hwnd, msg, w_param, l_param);
            return 0;
        }
        winuser::WM_COPYDATA => {
            let data = &*(l_param as *const COPYDATASTRUCT);

            let len = data.cbData as usize;
            let mut vec = Vec::with_capacity(len + size_of::<usize>());
            vec.extend_from_slice(slice::from_raw_parts(data.lpData.cast::<u8>(), len));
            vec.extend_from_slice(&data.dwData.to_ne_bytes());
            let mut vec = ManuallyDrop::new(vec);
            let bytes = vec.as_mut_ptr();
            winuser::PostMessageW(hwnd, messages::BYTES_ID, len, bytes as isize);
            return 0;
        }
        _ => {}
    }

    winuser::DefWindowProcW(hwnd, msg, w_param, l_param)
}

unsafe fn init_window(
    class_name: Vec<u16>,
    window_name: Option<Vec<u16>>,
) -> io::Result<(WindowClassHandle, WindowHandle)> {
    let wnd = winuser::WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: 0,
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
    };

    if winuser::RegisterClassW(&wnd) == 0 {
        return Err(io::Error::last_os_error());
    }

    let class = WindowClassHandle { class_name };

    let hwnd = winuser::CreateWindowExW(
        0,
        class.class_name.as_ptr(),
        window_name.map(|n| n.as_ptr()).unwrap_or_else(ptr::null),
        winuser::WS_DISABLED,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        ptr::null(),
    );

    if hwnd == 0 {
        return Err(io::Error::last_os_error());
    }

    let window = WindowHandle { hwnd };
    Ok((class, window))
}

/// A windows application window.
///
/// Note: repr(C) is important here to ensure drop order.
#[repr(C)]
pub(crate) struct WindowLoop {
    pub(crate) areas: Vec<AreaHandle>,
    pub(crate) window: WindowHandle,
    window_class: WindowClassHandle,
    events_rx: mpsc::UnboundedReceiver<WindowEvent>,
    thread: Option<thread::JoinHandle<Result<(), WindowError>>>,
}

impl WindowLoop {
    /// Construct a new window.
    pub(crate) async fn new(
        class_name: &OsStr,
        window_name: Option<&OsStr>,
        clipboard_events: bool,
        areas: Vec<AreaHandle>,
    ) -> Result<WindowLoop, WindowError> {
        let class_name = class_name.to_wide_null();
        let window_name = window_name.map(|n| n.to_wide_null());

        if class_name.len() > 256 {
            return Err(WindowError::ClassNameTooLong(class_name.len()));
        }

        let (return_tx, return_rx) = oneshot::channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let mut hmenus = Vec::with_capacity(areas.len());

        for menu in &areas {
            hmenus.push(
                menu.popup_menu
                    .as_ref()
                    .map(|p| (p.hmenu, p.open_menu.copy_data())),
            );
        }

        let thread = thread::spawn(move || unsafe {
            // NB: Don't move this, it's important that the window is
            // initialized in the background thread.
            let (window_class, window) =
                init_window(class_name, window_name).map_err(WindowError::Init)?;

            let mut clipboard_manager = if clipboard_events {
                if AddClipboardFormatListener(window.hwnd) == FALSE {
                    return Err(WindowError::AddClipboardFormatListener(
                        io::Error::last_os_error(),
                    ));
                }

                Some(ClipboardManager::new(&events_tx))
            } else {
                None
            };

            let mut menu_manager =
                (!hmenus.is_empty()).then(|| MenuManager::new(&events_tx, &hmenus));

            let hwnd = window.hwnd;

            if return_tx.send((window_class, window)).is_err() {
                return Ok(());
            }

            let mut msg = MaybeUninit::zeroed();

            while winuser::GetMessageW(msg.as_mut_ptr(), hwnd, 0, 0) != FALSE {
                let msg = &*msg.as_ptr();

                if let Some(clipboard_manager) = &mut clipboard_manager {
                    if clipboard_manager.dispatch(msg) {
                        continue;
                    }
                }

                if let Some(menu_manager) = &mut menu_manager {
                    if menu_manager.dispatch(msg) {
                        continue;
                    }
                }

                match msg.message {
                    winuser::WM_QUIT | winuser::WM_DESTROY => {
                        break;
                    }
                    messages::BYTES_ID => {
                        let len = msg.wParam;

                        let bytes = Vec::from_raw_parts(
                            msg.lParam as *mut u8,
                            len,
                            len + size_of::<usize>(),
                        );

                        let ty = bytes
                            .as_ptr()
                            .add(bytes.len())
                            .cast::<usize>()
                            .read_unaligned();

                        _ = events_tx.send(WindowEvent::CopyData(ty, bytes));
                        continue;
                    }
                    _ => {}
                }

                winuser::TranslateMessage(msg);
                winuser::DispatchMessageW(msg);
            }

            Ok(())
        });

        let Some((window_class, window)) = return_rx.await.ok() else {
            thread.join().map_err(|_| WindowError::ThreadPanicked)??;
            return Err(WindowError::ThreadExited);
        };

        Ok(WindowLoop {
            areas,
            window,
            window_class,
            events_rx,
            thread: Some(thread),
        })
    }

    /// Tick the window through a single event cycle.
    pub(crate) async fn tick(&mut self) -> WindowEvent {
        self.events_rx.recv().await.unwrap_or(WindowEvent::Shutdown)
    }

    /// Test if the window has been closed.
    pub(crate) fn is_closed(&self) -> bool {
        self.thread.is_none()
    }

    /// Join the current window.
    pub(crate) fn join(&mut self) -> Result<()> {
        if self.thread.is_none() {
            return Ok(());
        }

        let result = unsafe { winuser::PostMessageW(self.window.hwnd, winuser::WM_DESTROY, 0, 0) };

        if result == FALSE {
            return Err(Error::new(PostMessageDestroy));
        }

        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| ThreadError(WindowError::ThreadPanicked))?
                .map_err(ThreadError)?;
        }

        Ok(())
    }
}

impl Drop for WindowLoop {
    fn drop(&mut self) {
        for menu in &self.areas {
            _ = self.window.delete_notification(menu.area_id);
        }
    }
}
