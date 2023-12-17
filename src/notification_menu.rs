use crate::{Icon, MenuItem, Token};

/// A notification menu.
///
/// This is opened when you click on the window icon that lives in the system
/// tray.
#[derive(Default)]
pub struct NotificationMenu {
    pub(super) menu: Vec<MenuItem>,
    pub(super) initial_icon: Option<Icon>,
}

impl NotificationMenu {
    /// Construct a new empty notification menu.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the default icon to use.
    pub fn initial_icon(self, initial_icon: Icon) -> Self {
        Self {
            initial_icon: Some(initial_icon),
            ..self
        }
    }

    /// Push a new menu item.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{NotificationMenu, MenuItem};
    ///
    /// let mut menu = NotificationMenu::new();
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
