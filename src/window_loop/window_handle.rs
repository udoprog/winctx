use std::io;
use std::mem::{size_of, MaybeUninit};

use windows_sys::Win32::Foundation::{FALSE, HWND};
use windows_sys::Win32::UI::Shell::{self as shellapi, SHGetStockIconInfo};

use crate::convert::copy_wstring_lossy;
use crate::notification::NotificationIcon;
use crate::{AreaId, Notification};

use super::{messages, IconHandle};

pub(crate) struct WindowHandle {
    pub(super) hwnd: HWND,
}

impl WindowHandle {
    fn new_nid(&self, area_id: AreaId) -> shellapi::NOTIFYICONDATAW {
        let mut nid: shellapi::NOTIFYICONDATAW = unsafe { MaybeUninit::zeroed().assume_init() };
        nid.cbSize = size_of::<shellapi::NOTIFYICONDATAW>() as u32;
        nid.hWnd = self.hwnd;
        nid.uID = area_id.id();
        nid
    }

    pub(crate) fn add_notification(&mut self, area_id: AreaId) -> io::Result<()> {
        let mut nid = self.new_nid(area_id);
        nid.uFlags = shellapi::NIF_MESSAGE;
        nid.uCallbackMessage = messages::ICON_ID;

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub(crate) fn delete_notification(&mut self, area_id: AreaId) -> io::Result<()> {
        let result = unsafe {
            let mut nid = self.new_nid(area_id);
            nid.uFlags = shellapi::NIF_ICON;
            shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &nid)
        };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Clear out tooltip.
    pub(crate) fn modify_notification(
        &self,
        area_id: AreaId,
        icon: Option<&IconHandle>,
        tooltip: Option<&str>,
    ) -> io::Result<()> {
        let mut nid = self.new_nid(area_id);

        if let Some(icon) = icon {
            nid.uFlags |= shellapi::NIF_ICON;
            nid.hIcon = icon.hicon;
        }

        if let Some(tooltip) = tooltip {
            nid.uFlags |= shellapi::NIF_TIP | shellapi::NIF_SHOWTIP;
            copy_wstring_lossy(&mut nid.szTip, tooltip);
        }

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Send a notification.
    pub(crate) fn send_notification(&self, area_id: AreaId, n: Notification) -> io::Result<()> {
        let mut nid = self.new_nid(area_id);
        nid.uFlags = shellapi::NIF_INFO;

        if let Some(title) = n.title {
            copy_wstring_lossy(&mut nid.szInfoTitle, title.as_str());
        }

        if let Some(message) = n.message {
            copy_wstring_lossy(&mut nid.szInfo, message.as_str());
        }

        if let Some(timeout) = n.timeout {
            nid.Anonymous.uTimeout = timeout.as_millis() as u32;
        }

        nid.dwInfoFlags = n.options;

        if let Some(icon) = n.icon {
            match icon {
                NotificationIcon::Info => {
                    nid.dwInfoFlags |= shellapi::NIIF_INFO;
                }
                NotificationIcon::Warning => {
                    nid.dwInfoFlags |= shellapi::NIIF_WARNING;
                }
                NotificationIcon::Error => {
                    nid.dwInfoFlags |= shellapi::NIIF_ERROR;
                }
                NotificationIcon::StockIcon(stock) => unsafe {
                    let mut sii: shellapi::SHSTOCKICONINFO = MaybeUninit::zeroed().assume_init();
                    sii.cbSize = size_of::<shellapi::SHSTOCKICONINFO>() as u32;

                    let mut opts = shellapi::SHGSI_ICON | n.stock_icon_opts;

                    if nid.dwInfoFlags & shellapi::NIIF_LARGE_ICON != 0 {
                        opts |= shellapi::SHGSI_LARGEICON;
                    } else {
                        opts |= shellapi::SHGSI_SMALLICON;
                    }

                    if SHGetStockIconInfo(stock.as_id(), opts, &mut sii) == 0 {
                        nid.hBalloonIcon = sii.hIcon;
                        nid.dwInfoFlags |= shellapi::NIIF_USER;
                    }
                },
            };
        }

        let result = unsafe { shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &nid) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}
