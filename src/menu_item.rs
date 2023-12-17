//! Types related to menu construction.

use crate::{ItemId, ModifyMenuItem};

pub(super) enum MenuItemKind {
    Separator,
    String { text: String },
}

/// A menu item in the context menu.
///
/// This is constructed through:
/// * [`MenuItem::separator`].
/// * [`MenuItem::entry`].
pub struct MenuItem {
    pub(crate) item_id: ItemId,
    pub(crate) kind: MenuItemKind,
    pub(crate) initial: ModifyMenuItem,
}

impl MenuItem {
    pub(super) fn new(item_id: ItemId, kind: MenuItemKind) -> Self {
        Self {
            item_id,
            kind,
            initial: ModifyMenuItem::default(),
        }
    }

    /// Get the identifier of the menu item.
    pub fn id(&self) -> ItemId {
        self.item_id
    }

    /// Set the checked state of the menu item.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::WindowBuilder;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");;
    /// let area = window.new_area();
    ///
    /// let mut menu = area.popup_menu();
    /// menu.push_entry("Example Application").checked(true);
    /// ```
    pub fn checked(&mut self, checked: bool) -> &mut Self {
        self.initial.checked(checked);
        self
    }

    /// Set that the menu item should be highlighted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::WindowBuilder;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");;
    /// let area = window.new_area();
    ///
    /// let mut menu = area.popup_menu();
    /// menu.push_entry("Example Application").checked(true);
    /// ```
    pub fn highlight(&mut self, highlight: bool) -> &mut Self {
        self.initial.highlight(highlight);
        self
    }
}
