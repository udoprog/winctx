//! Types related to menu construction.

use std::fmt;

pub(crate) enum MenuItemKind {
    Separator,
    MenyEntry { text: String, default: bool },
}

/// A menu item in the context menu.
///
/// This is constructed through:
/// * [`MenuItem::separator`].
/// * [`MenuItem::entry`].
pub struct MenuItem {
    pub(crate) kind: MenuItemKind,
}

impl MenuItem {
    /// Construct a menu separator.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{NotificationMenu, MenuItem};
    ///
    /// let mut menu = NotificationMenu::new();
    /// menu.push(MenuItem::separator());
    /// ```
    pub fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
        }
    }

    /// Construct a menu entry.
    ///
    /// The `default` parameter indicates whether the entry shoudl be
    /// highlighted.
    ///
    /// This returns a token which can be matched against the token returned in
    /// [`Event::MenuItemClicked`].
    ///
    /// [`Event::MenuItemClicked`]: crate::Event::MenuItemClicked
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{NotificationMenu, MenuItem};
    ///
    /// let mut menu = NotificationMenu::new();
    /// menu.push(MenuItem::entry("Example Application", true));
    /// menu.push(MenuItem::entry("Exit...", false));
    /// ```
    pub fn entry<T>(text: T, default: bool) -> Self
    where
        T: fmt::Display,
    {
        Self {
            kind: MenuItemKind::MenyEntry {
                text: text.to_string(),
                default,
            },
        }
    }
}
