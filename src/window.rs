#![allow(clippy::field_reassign_with_default)]

use std::cell::UnsafeCell;
use std::io;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;

use tokio::sync::{mpsc, oneshot};
use windows_sys::Win32::Foundation::{FALSE, HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::HBRUSH;
use windows_sys::Win32::UI::Shell as shellapi;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::{HICON, HMENU};

use crate::convert::ToWide;
use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::{Notification, Result};

const ICON_MSG_ID: u32 = winuser::WM_USER + 1;

// Stash that is shared with the window process.
//
// The safety of this is a little bit tricky, but the interior data is
// uninitialized, and it can only ever be owned by one window thread at a time
// since it's stashed in a thread-local.
thread_local! {
    static STASH: UnsafeCell<ptr::NonNull<Stash>> = UnsafeCell::new(ptr::NonNull::dangling());
}

struct Stash {
    info: WindowInfo,
    tx: mpsc::UnboundedSender<WindowEvent>,
}

impl Stash {
    unsafe fn install(&mut self) -> DropStash<'_> {
        // Drop the stash.
        STASH.with(|stash| {
            *stash.get() = ptr::NonNull::from(self);
        });

        DropStash(PhantomData)
    }
}

struct DropStash<'a>(PhantomData<&'a mut Stash>);

impl Drop for DropStash<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            STASH.with(|stash| {
                *stash.get() = ptr::NonNull::dangling();
            });
        }
    }
}

/// Copy a wide string from a source to a destination.
pub(crate) fn copy_wstring(dest: &mut [u16], source: &str) {
    let source = source.to_wide_null();
    let len = usize::min(source.len(), dest.len());
    dest[..len].copy_from_slice(&source[..len]);
}

#[derive(Clone)]
struct WindowInfo {
    hwnd: HWND,
    hmenu: HMENU,
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
        nid.uCallbackMessage = ICON_MSG_ID;

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
    MenuClicked(u32),
    /// Shutdown was requested.
    Shutdown,
    /// Balloon was clicked.
    BalloonClicked,
    /// Balloon timed out.
    BalloonTimeout,
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let stash = &*STASH.with(|with| (*with.get()).as_ptr());

    match msg {
        ICON_MSG_ID => {
            match l_param as u32 {
                // Balloon clicked.
                shellapi::NIN_BALLOONUSERCLICK => {
                    _ = stash.tx.send(WindowEvent::BalloonClicked);
                    return 0;
                }
                // Balloon timed out.
                shellapi::NIN_BALLOONTIMEOUT => {
                    _ = stash.tx.send(WindowEvent::BalloonTimeout);
                    return 0;
                }
                winuser::WM_LBUTTONUP | winuser::WM_RBUTTONUP => {
                    let mut p = MaybeUninit::zeroed();

                    if winuser::GetCursorPos(p.as_mut_ptr()) == FALSE {
                        return 1;
                    }

                    let p = p.assume_init();

                    winuser::SetForegroundWindow(hwnd);

                    winuser::TrackPopupMenu(
                        stash.info.hmenu,
                        0,
                        p.x,
                        p.y,
                        (winuser::TPM_BOTTOMALIGN | winuser::TPM_LEFTALIGN) as i32,
                        hwnd,
                        ptr::null_mut(),
                    );

                    return 0;
                }
                _ => (),
            }
        }
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(0);
            return 0;
        }
        winuser::WM_MENUCOMMAND => {
            let menu_id = winuser::GetMenuItemID(stash.info.hmenu, w_param as i32) as i32;

            if menu_id != -1 {
                _ = stash.tx.send(WindowEvent::MenuClicked(menu_id as u32));
            }

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
        hIcon: winuser::LoadIconW(0, winuser::IDI_APPLICATION),
        hCursor: winuser::LoadCursorW(0, winuser::IDI_APPLICATION),
        hbrBackground: 16 as HBRUSH,
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
        winuser::WS_OVERLAPPEDWINDOW,
        winuser::CW_USEDEFAULT,
        0,
        winuser::CW_USEDEFAULT,
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
pub(crate) struct Window {
    info: WindowInfo,
    events_rx: mpsc::UnboundedReceiver<WindowEvent>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Window {
    /// Construct a new window.
    pub(crate) async fn new(class_name: &str, name: &str) -> io::Result<Window> {
        let class_name = class_name.to_wide_null();
        let name = name.to_wide_null();

        let (tx, rx) = oneshot::channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let thread = thread::spawn(move || unsafe {
            // NB: Don't move this, it's important that the window is
            // initialized in the background thread.
            let info = match init_window(class_name, name) {
                Ok(info) => info,
                Err(e) => {
                    _ = tx.send(Err(e));
                    return;
                }
            };

            if tx.send(Ok(info.clone())).is_err() {
                panic!("failed to send window information to parent thread");
            }

            let mut stash = Stash {
                info: info.clone(),
                tx: events_tx,
            };

            let _stash = stash.install();

            let mut msg = MaybeUninit::<winuser::MSG>::zeroed();

            loop {
                let ret = winuser::GetMessageW(msg.as_mut_ptr(), info.hwnd, 0, 0);

                {
                    let msg = &*msg.as_ptr();

                    if ret == 0
                        || msg.message == winuser::WM_QUIT
                        || msg.message == winuser::WM_DESTROY
                    {
                        break;
                    }
                }

                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageW(msg.as_ptr());
            }

            _ = info.delete_icon();
        });

        let info = rx
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "canceled"))??;

        let w = Window {
            info,
            events_rx,
            thread: Some(thread),
        };

        Ok(w)
    }

    /// Tick the window through a single event cycle.
    pub(crate) async fn tick(&mut self) -> WindowEvent {
        self.events_rx.recv().await.unwrap_or(WindowEvent::Shutdown)
    }

    /// Test if the window has been closed.
    pub(super) fn is_closed(&self) -> bool {
        self.thread.is_none()
    }

    /// Join the current window.
    pub(crate) fn join(&mut self) -> Result<()> {
        let result = unsafe { winuser::PostMessageW(self.info.hwnd, winuser::WM_DESTROY, 0, 0) };

        if result == FALSE {
            return Err(Error::new(PostMessageDestroy));
        }

        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| WindowThreadPanicked)?;
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

        let result = unsafe { winuser::InsertMenuItemW(self.info.hmenu, item_idx, 1, &item) };

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
        let mut nid = self.info.new_nid();
        nid.uFlags = shellapi::NIF_INFO;

        if let Some(title) = n.title {
            copy_wstring(&mut nid.szInfoTitle, title.as_str());
        }

        copy_wstring(&mut nid.szInfo, n.message.as_str());

        if let Some(timeout) = n.timeout {
            nid.Anonymous.uTimeout = timeout.as_millis() as u32;
        }

        nid.dwInfoFlags = n.icon.into_flags();
        nid.uCallbackMessage = token;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Set an icon from a buffer.
    pub(crate) fn set_icon_from_buffer(
        &self,
        buffer: &[u8],
        width: u32,
        height: u32,
    ) -> io::Result<()> {
        let offset = unsafe {
            winuser::LookupIconIdFromDirectoryEx(
                buffer.as_ptr(),
                TRUE,
                width as i32,
                height as i32,
                winuser::LR_DEFAULTCOLOR,
            )
        };

        if offset == 0 {
            return Err(io::Error::last_os_error());
        }

        let icon_data = &buffer[offset as usize..];

        let hicon = unsafe {
            winuser::CreateIconFromResourceEx(
                icon_data.as_ptr(),
                icon_data.len() as u32,
                TRUE,
                0x30000,
                width as i32,
                height as i32,
                winuser::LR_DEFAULTCOLOR,
            )
        };

        if hicon == 0 {
            return Err(io::Error::last_os_error());
        }

        self.set_icon(hicon)
    }

    /// Internal call to set icon.
    fn set_icon(&self, icon: HICON) -> io::Result<()> {
        let result = unsafe {
            let mut nid = self.info.new_nid();
            nid.uFlags = shellapi::NIF_ICON;
            nid.hIcon = icon;

            shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid)
        };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}
