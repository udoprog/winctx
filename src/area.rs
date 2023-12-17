use std::fmt;

use crate::{AreaId, Icon, ModifyArea, PopupMenu};

/// A notification area.
///
/// This is opened when you click on the window icon that lives in the system
/// tray.
pub struct Area {
    pub(super) id: AreaId,
    pub(super) popup_menu: Option<PopupMenu>,
    pub(super) initial: ModifyArea,
}

impl Area {
    /// Construct a new empty notification area.
    ///
    /// Without any configuration this will just occupy a blank space in the
    /// notification area.
    ///
    /// To set an icon or a popup menu, use the relevant builder methods.
    pub fn new(area_id: AreaId) -> Self {
        Self {
            id: area_id,
            popup_menu: None,
            initial: ModifyArea::default(),
        }
    }

    /// Get the area identifier.
    pub fn id(&self) -> AreaId {
        self.id
    }

    /// Set the icon of the notification area.
    #[inline]
    pub fn icon(&mut self, icon: Icon) -> &mut Self {
        self.initial.icon(icon);
        self
    }

    /// Set the tooltip of the notification area.
    #[inline]
    pub fn tooltip<T>(&mut self, tooltip: T) -> &mut Self
    where
        T: fmt::Display,
    {
        self.initial.tooltip(tooltip);
        self
    }

    /// Set that a popup menu should be used and return a handle to populate it.
    #[inline]
    pub fn popup_menu(&mut self) -> &mut PopupMenu {
        if self.popup_menu.is_none() {
            self.popup_menu = Some(PopupMenu::new(self.id));
        }

        self.popup_menu.as_mut().unwrap()
    }
}
