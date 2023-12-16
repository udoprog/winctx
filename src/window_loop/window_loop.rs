#![allow(clippy::field_reassign_with_default)]

use std::io;
use std::mem::size_of;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::str;
use std::thread;

use tokio::sync::mpsc;
use tokio::sync::oneshot;
use windows_sys::Win32::Foundation::{FALSE, HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows_sys::Win32::System::DataExchange::AddClipboardFormatListener;
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

use crate::convert::ToWide;
use crate::error::ErrorKind::*;
use crate::error::{Error, WindowError};
use crate::event_loop::ClipboardEvent;
use crate::notification::NotificationIcon;
use crate::window_loop::messages;
use crate::{Notification, Result};

use super::{ClipboardManager, Icon, MenuManager};

/// Copy a wide string from a source to a destination.
pub(crate) fn copy_wstring(dest: &mut [u16], source: &str) {
    let source = source.to_wide_null();
    let len = usize::min(source.len(), dest.len());
    dest[..len].copy_from_slice(&source[..len]);
}

#[derive(Clone)]
struct WindowInfo {
    hwnd: HWND,
    hmenu: winuser::HMENU,
}

impl WindowInfo {
    fn new_nid(&self) -> shellapi::NOTIFYICONDATAW {
        let mut nid: shellapi::NOTIFYICONDATAW = unsafe { MaybeUninit::zeroed().assume_init() };
        nid.cbSize = size_of::<shellapi::NOTIFYICONDATAW>() as u32;
        nid.hWnd = self.hwnd;
        nid.uID = 0x1;
        nid
    }

    fn add_icon(&self) -> io::Result<()> {
        let mut nid = self.new_nid();
        nid.uFlags = shellapi::NIF_MESSAGE;
        nid.uCallbackMessage = messages::ICON_ID;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    fn delete_icon(&self) -> io::Result<()> {
        let result = unsafe {
            let mut nid = self.new_nid();
            nid.uFlags = shellapi::NIF_ICON;
            shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &nid)
        };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

unsafe impl Send for WindowInfo {}
unsafe impl Sync for WindowInfo {}

#[derive(Debug)]
pub(crate) enum WindowEvent {
    /// A meny item was clicked.
    MenuItemClicked(u32),
    /// Shutdown was requested.
    Shutdown,
    /// Clipboard event.
    Clipboard(ClipboardEvent),
    /// Balloon was clicked.
    NotificationClicked,
    /// Balloon timed out.
    NotificationDismissed,
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

fn new_menuitem() -> winuser::MENUITEMINFOW {
    let mut info: winuser::MENUITEMINFOW = unsafe { MaybeUninit::zeroed().assume_init() };
    info.cbSize = size_of::<winuser::MENUITEMINFOW>() as u32;
    info
}

unsafe fn init_window(class_name: Vec<u16>, name: Vec<u16>) -> io::Result<WindowInfo> {
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

    let hwnd = winuser::CreateWindowExW(
        0,
        class_name.as_ptr(),
        name.as_ptr(),
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

    // Setup menu
    let hmenu = winuser::CreatePopupMenu();

    let m = winuser::MENUINFO {
        cbSize: size_of::<winuser::MENUINFO>() as u32,
        fMask: winuser::MIM_APPLYTOSUBMENUS | winuser::MIM_STYLE,
        dwStyle: winuser::MNS_NOTIFYBYPOS,
        cyMax: 0,
        hbrBack: 0,
        dwContextHelpID: 0,
        dwMenuData: 0,
    };

    if winuser::SetMenuInfo(hmenu, &m) == FALSE {
        return Err(io::Error::last_os_error());
    }

    let info = WindowInfo { hwnd, hmenu };
    info.add_icon()?;
    Ok(info)
}

/// A windows application window.
pub(crate) struct WindowLoop {
    info: WindowInfo,
    events_rx: mpsc::UnboundedReceiver<WindowEvent>,
    thread: Option<thread::JoinHandle<Result<(), WindowError>>>,
    icon: Option<Icon>,
}

impl WindowLoop {
    /// Construct a new window.
    pub(crate) async fn new(
        class_name: &str,
        name: &str,
        clipboard_events: bool,
    ) -> Result<WindowLoop, WindowError> {
        let class_name = class_name.to_wide_null();
        let name = name.to_wide_null();

        let (return_tx, return_rx) = oneshot::channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let thread = thread::spawn(move || unsafe {
            // NB: Don't move this, it's important that the window is
            // initialized in the background thread.
            let info = init_window(class_name, name).map_err(WindowError::Init)?;

            let mut clipboard_manager = if clipboard_events {
                if AddClipboardFormatListener(info.hwnd) == FALSE {
                    return Err(WindowError::AddClipboardFormatListener(
                        io::Error::last_os_error(),
                    ));
                }

                Some(ClipboardManager::new(&events_tx))
            } else {
                None
            };

            let mut menu_manager = MenuManager::new(&events_tx, info.hmenu);

            if return_tx.send(info.clone()).is_err() {
                return Ok(());
            }

            let mut msg = MaybeUninit::zeroed();

            while winuser::GetMessageW(msg.as_mut_ptr(), info.hwnd, 0, 0) != FALSE {
                let msg = &*msg.as_ptr();

                if let Some(clipboard_manager) = &mut clipboard_manager {
                    if clipboard_manager.dispatch(msg) {
                        continue;
                    }
                }

                if menu_manager.dispatch(msg) {
                    continue;
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

                        let _ = events_tx.send(WindowEvent::CopyData(ty, bytes));
                        continue;
                    }
                    _ => {}
                }

                winuser::TranslateMessage(msg);
                winuser::DispatchMessageW(msg);
            }

            info.delete_icon().map_err(WindowError::DeleteIcon)?;
            Ok(())
        });

        let Some(info) = return_rx.await.ok() else {
            thread.join().map_err(|_| WindowError::ThreadPanicked)??;
            return Err(WindowError::ThreadExited);
        };

        let w = WindowLoop {
            info,
            events_rx,
            thread: Some(thread),
            icon: None,
        };

        Ok(w)
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
        let result = unsafe { winuser::PostMessageW(self.info.hwnd, winuser::WM_DESTROY, 0, 0) };

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

    /// Set tooltip.
    pub(crate) fn set_tooltip(&self, tooltip: &str) -> io::Result<()> {
        let mut nid = self.info.new_nid();
        copy_wstring(&mut nid.szTip, tooltip);

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Add a menu entry.
    pub(crate) fn add_menu_entry(
        &self,
        item_idx: u32,
        item_name: &str,
        default: bool,
    ) -> io::Result<()> {
        let mut st = item_name.to_wide_null();
        let mut item = new_menuitem();
        item.fMask =
            winuser::MIIM_FTYPE | winuser::MIIM_STRING | winuser::MIIM_ID | winuser::MIIM_STATE;
        item.fType = winuser::MFT_STRING;
        item.wID = item_idx;
        item.dwTypeData = st.as_mut_ptr();
        item.cch = (item_name.len() * 2) as u32;

        if default {
            item.fState = winuser::MFS_DEFAULT;
        }

        let result = unsafe { winuser::InsertMenuItemW(self.info.hmenu, item_idx, TRUE, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Add a menu separator at the given index.
    pub(crate) fn add_menu_separator(&self, item_idx: u32) -> io::Result<()> {
        let mut item = new_menuitem();
        item.fMask = winuser::MIIM_FTYPE;
        item.fType = winuser::MFT_SEPARATOR;
        item.wID = item_idx;

        let result = unsafe { winuser::InsertMenuItemW(self.info.hmenu, item_idx, 1, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Send a notification.
    pub(crate) fn send_notification(&self, token: u32, n: Notification) -> io::Result<()> {
        /// Convert into a flag.
        fn into_flags(icon: NotificationIcon) -> u32 {
            match icon {
                NotificationIcon::Info => shellapi::NIIF_INFO,
                NotificationIcon::Error => shellapi::NIIF_ERROR,
                NotificationIcon::Warning => shellapi::NIIF_WARNING,
            }
        }

        let mut nid = self.info.new_nid();
        nid.uFlags = shellapi::NIF_INFO;

        if let Some(title) = n.title {
            copy_wstring(&mut nid.szInfoTitle, title.as_str());
        }

        copy_wstring(&mut nid.szInfo, n.message.as_str());

        if let Some(timeout) = n.timeout {
            nid.Anonymous.uTimeout = timeout.as_millis() as u32;
        }

        nid.dwInfoFlags = into_flags(n.icon);
        nid.uCallbackMessage = token;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Set context icon.
    pub(crate) fn set_icon(&mut self, icon: Icon) -> io::Result<()> {
        let result = unsafe {
            let mut nid = self.info.new_nid();
            nid.uFlags = shellapi::NIF_ICON;
            nid.hIcon = icon.as_raw_handle();
            shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid)
        };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        self.icon = Some(icon);
        Ok(())
    }
}
