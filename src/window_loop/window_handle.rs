use std::io;
use std::mem::{size_of, MaybeUninit};

use windows_sys::Win32::Foundation::{FALSE, HWND};
use windows_sys::Win32::UI::Shell as shellapi;

use crate::convert::copy_wstring_lossy;
use crate::notification::NotificationIcon;
use crate::Notification;

use super::{messages, IconHandle};

pub(crate) struct WindowHandle {
    pub(super) hwnd: HWND,
}

impl WindowHandle {
    fn new_nid(&self) -> shellapi::NOTIFYICONDATAW {
        let mut nid: shellapi::NOTIFYICONDATAW = unsafe { MaybeUninit::zeroed().assume_init() };
        nid.cbSize = size_of::<shellapi::NOTIFYICONDATAW>() as u32;
        nid.hWnd = self.hwnd;
        nid.uID = 0x1;
        nid
    }

    pub(crate) fn add_icon(&mut self) -> io::Result<()> {
        let mut nid = self.new_nid();
        nid.uFlags = shellapi::NIF_MESSAGE;
        nid.uCallbackMessage = messages::ICON_ID;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub(crate) fn delete_icon(&mut self) -> io::Result<()> {
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

    /// Clear out tooltip.
    pub(crate) fn clear_tooltip(&self) -> io::Result<()> {
        let mut nid = self.new_nid();
        nid.uFlags = shellapi::NIF_TIP | shellapi::NIF_SHOWTIP;
        copy_wstring_lossy(&mut nid.szTip, "");

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Set tooltip.
    pub(crate) fn set_tooltip(&self, tooltip: &str) -> io::Result<()> {
        let mut nid = self.new_nid();
        nid.uFlags = shellapi::NIF_TIP | shellapi::NIF_SHOWTIP;
        copy_wstring_lossy(&mut nid.szTip, tooltip);

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Set context icon.
    pub(crate) fn set_icon(&mut self, icon: &IconHandle) -> io::Result<()> {
        let result = unsafe {
            let mut nid = self.new_nid();
            nid.uFlags = shellapi::NIF_ICON;
            nid.hIcon = icon.hicon;
            shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid)
        };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Send a notification.
    pub(crate) fn send_notification(&self, token: u32, n: Notification) -> io::Result<()> {
        /// Convert into a flag.
        fn into_flags(options: u32, icon: NotificationIcon) -> u32 {
            let icon = match icon {
                NotificationIcon::Info => shellapi::NIIF_INFO,
                NotificationIcon::Error => shellapi::NIIF_ERROR,
                NotificationIcon::Warning => shellapi::NIIF_WARNING,
            };

            options | icon
        }

        let mut nid = self.new_nid();
        nid.uFlags = shellapi::NIF_INFO;

        if let Some(title) = n.title {
            copy_wstring_lossy(&mut nid.szInfoTitle, title.as_str());
        }

        copy_wstring_lossy(&mut nid.szInfo, n.message.as_str());

        if let Some(timeout) = n.timeout {
            nid.Anonymous.uTimeout = timeout.as_millis() as u32;
        }

        nid.dwInfoFlags = into_flags(n.options, n.icon);
        nid.uCallbackMessage = token;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}
