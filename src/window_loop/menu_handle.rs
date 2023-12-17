use std::io;
use std::mem::{size_of, MaybeUninit};
use std::str;

use windows_sys::Win32::Foundation::{FALSE, TRUE};
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

use crate::convert::ToWide;

pub(crate) struct MenuHandle {
    pub(super) hmenu: winuser::HMENU,
}

impl MenuHandle {
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

        let result = unsafe { winuser::InsertMenuItemW(self.hmenu, item_idx, TRUE, &item) };

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

        let result = unsafe { winuser::InsertMenuItemW(self.hmenu, item_idx, 1, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for MenuHandle {
    fn drop(&mut self) {
        unsafe {
            _ = winuser::DestroyMenu(self.hmenu);
        }
    }
}

fn new_menuitem() -> winuser::MENUITEMINFOW {
    let mut info: winuser::MENUITEMINFOW = unsafe { MaybeUninit::zeroed().assume_init() };
    info.cbSize = size_of::<winuser::MENUITEMINFOW>() as u32;
    info
}
