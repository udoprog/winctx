use crate::{MenuItem, MenuItemId};

/// The structure of a popup menu.
#[derive(Default)]
pub struct PopupMenu {
    pub(super) menu: Vec<MenuItem>,
    /// The default item in the menu.
    pub(super) default: Option<MenuItemId>,
}

impl PopupMenu {
    /// Construct a new empt popup menu.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new menu item.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{PopupMenu, MenuItem};
    ///
    /// let mut menu = PopupMenu::new();
    /// menu.push(MenuItem::entry("Example Application"));
    /// menu.push(MenuItem::separator());
    /// menu.push(MenuItem::entry("Exit..."));
    /// ```
    pub fn push(&mut self, menu_item: MenuItem) -> MenuItemId {
        let token = MenuItemId::new(self.menu.len() as u32);
        self.menu.push(menu_item);
        token
    }

    /// Set the default item in the menu.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{PopupMenu, MenuItem};
    ///
    /// let mut menu = PopupMenu::new();
    /// let first = menu.push(MenuItem::entry("Example Application"));
    /// menu.push(MenuItem::separator());
    /// menu.push(MenuItem::entry("Exit..."));
    /// menu.set_default(first);
    /// ```
    pub fn set_default(&mut self, menu_item_id: MenuItemId) {
        self.default = Some(menu_item_id);
    }
}
