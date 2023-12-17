use crate::{MenuItem, Token};

/// The structure of a popup menu.
#[derive(Default)]
pub struct PopupMenu {
    pub(super) menu: Vec<MenuItem>,
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
    /// menu.push(MenuItem::entry("Example Application", true));
    /// menu.push(MenuItem::separator());
    /// menu.push(MenuItem::entry("Exit...", false));
    /// ```
    pub fn push(&mut self, menu_item: MenuItem) -> Token {
        let token = Token::new(self.menu.len() as u32);
        self.menu.push(menu_item);
        token
    }
}
