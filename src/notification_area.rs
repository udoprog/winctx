use crate::{ModifyArea, PopupMenu};

/// A notification area.
///
/// This is opened when you click on the window icon that lives in the system
/// tray.
#[derive(Default)]
pub struct NotificationArea {
    pub(super) popup_menu: Option<PopupMenu>,
    pub(super) initial: Option<ModifyArea>,
}

impl NotificationArea {
    /// Construct a new empty notification area.
    ///
    /// Without any configuration this will just occupy a blank space in the
    /// notification area.
    ///
    /// To set an icon or a popup menu, use the relevant builder methods.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the initial state of the notification area.
    pub fn initial(self, initial_icon: ModifyArea) -> Self {
        Self {
            initial: Some(initial_icon),
            ..self
        }
    }

    /// Set a popup menu that should be used.
    pub fn popup_menu(self, popup_menu: PopupMenu) -> Self {
        Self {
            popup_menu: Some(popup_menu),
            ..self
        }
    }
}
