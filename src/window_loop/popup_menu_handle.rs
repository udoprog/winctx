use std::io;
use std::mem::{size_of, MaybeUninit};
use std::str;

use windows_sys::Win32::Foundation::{FALSE, TRUE};
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

use crate::convert::ToWide;
use crate::ModifyMenuItem;

#[repr(C)]
pub(crate) struct PopupMenuHandle {
    pub(crate) hmenu: winuser::HMENU,
}

impl PopupMenuHandle {
    /// Construct a new menu handle.
    pub(crate) fn new() -> io::Result<Self> {
        unsafe {
            // Setup menu
            let hmenu = winuser::CreatePopupMenu();

            if hmenu == 0 {
                return Err(io::Error::last_os_error());
            }

            let menu = Self { hmenu };

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
        menu_item_id: u32,
        string: &str,
        default: bool,
        modify: &ModifyMenuItem,
    ) -> io::Result<()> {
        let mut item = new_menuitem();
        item.fMask = winuser::MIIM_FTYPE | winuser::MIIM_ID;
        item.fType = winuser::MFT_STRING;
        item.wID = menu_item_id;

        let string = string.to_wide_null();

        modify_string(&mut item, Some(&string[..]));
        modify_default(&mut item, default);
        apply(&mut item, modify);

        let result = unsafe { winuser::InsertMenuItemW(self.hmenu, menu_item_id, TRUE, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Add a menu separator at the given index.
    pub(crate) fn add_menu_separator(
        &self,
        menu_item_id: u32,
        default: bool,
        modify: &ModifyMenuItem,
    ) -> io::Result<()> {
        let mut item = new_menuitem();
        item.fMask = winuser::MIIM_FTYPE | winuser::MIIM_ID;
        item.fType = winuser::MFT_SEPARATOR;
        item.wID = menu_item_id;

        apply(&mut item, modify);
        modify_default(&mut item, default);

        let result = unsafe { winuser::InsertMenuItemW(self.hmenu, menu_item_id, 1, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Set the checked state of the specified menu item.
    pub(crate) fn modify_menu_item(
        &self,
        item_idx: u32,
        modify: &ModifyMenuItem,
    ) -> io::Result<()> {
        let mut item = new_menuitem();
        apply(&mut item, modify);

        let result = unsafe { winuser::SetMenuItemInfoW(self.hmenu, item_idx, 1, &item) };

        if result == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

fn modify_string(item: &mut winuser::MENUITEMINFOW, string: Option<&[u16]>) {
    if let Some(string) = string {
        item.fMask |= winuser::MIIM_STRING;
        item.dwTypeData = string.as_ptr() as *mut _;
        item.cch = string.len().saturating_sub(1) as u32 * 2;
    }
}

fn modify_default(item: &mut winuser::MENUITEMINFOW, default: bool) {
    if default {
        item.fMask |= winuser::MIIM_STATE;
        item.fState |= winuser::MFS_DEFAULT;
    }
}

fn apply(item: &mut winuser::MENUITEMINFOW, modify: &ModifyMenuItem) {
    modify_checked(item, modify.checked);
    modify_highlight(item, modify.highlight);
}

fn modify_checked(item: &mut winuser::MENUITEMINFOW, checked: Option<bool>) {
    if let Some(checked) = checked {
        item.fMask |= winuser::MIIM_STATE;

        item.fState |= if checked {
            winuser::MFS_CHECKED
        } else {
            winuser::MFS_UNCHECKED
        };
    }
}

fn modify_highlight(item: &mut winuser::MENUITEMINFOW, highlight: Option<bool>) {
    if let Some(highlight) = highlight {
        item.fMask |= winuser::MIIM_STATE;

        item.fState |= if highlight {
            winuser::MFS_HILITE
        } else {
            winuser::MFS_UNHILITE
        };
    }
}

impl Drop for PopupMenuHandle {
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
