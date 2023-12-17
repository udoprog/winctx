mod messages;

pub(super) use self::window_loop::{WindowEvent, WindowLoop};
mod window_loop;

pub(crate) use self::icon::Icon;
mod icon;

use self::clipboard_manager::ClipboardManager;
mod clipboard_manager;

use self::menu_manager::MenuManager;
mod menu_manager;

use self::window_handle::WindowHandle;
mod window_handle;

use self::window_class_handle::WindowClassHandle;
mod window_class_handle;

use self::menu_handle::MenuHandle;
mod menu_handle;

#[derive(Default)]
pub(crate) struct Icons {
    pub(crate) icon: Option<Icon>,
    pub(crate) error_icon: Option<Icon>,
}
