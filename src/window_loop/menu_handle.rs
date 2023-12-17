use std::io;
use std::mem::{size_of, MaybeUninit};
use std::str;

use windows_sys::Win32::Foundation::{FALSE, TRUE};
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

use crate::convert::ToWide;

/// The identifier for a notification menu.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MenuId(u32);

impl MenuId {
    /// Construct a new menu id.
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the menu id.
    pub(crate) fn id(&self) -> u32 {
        self.0
    }
}

#[repr(C)]
pub(crate) struct MenuHandle {
    pub(crate) id: MenuId,
    pub(crate) hmenu: winuser::HMENU,
    pub(crate) initial_icon: Option<usize>,
}

impl MenuHandle {
    /// Construct a new menu handle.
    pub(crate) fn new(id: MenuId, initial_icon: Option<usize>) -> io::Result<Self> {
        unsafe {
            // Setup menu
            let hmenu = winuser::CreatePopupMenu();

            if hmenu == 0 {
                return Err(io::Error::last_os_error());
            }

            let menu = Self {
                id,
                hmenu,
                initial_icon,
            };

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

            Ok(menu)
        }
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
