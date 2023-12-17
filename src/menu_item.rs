//! Types related to menu construction.

use std::fmt;

use crate::ModifyMenuItem;

pub(crate) enum MenuItemKind {
    Separator,
    MenyEntry { text: String },
}

/// A menu item in the context menu.
///
/// This is constructed through:
/// * [`MenuItem::separator`].
/// * [`MenuItem::entry`].
pub struct MenuItem {
    pub(crate) kind: MenuItemKind,
    pub(crate) initial: ModifyMenuItem,
}

impl MenuItem {
    /// Construct a menu separator.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{PopupMenu, MenuItem};
    ///
    /// let mut menu = PopupMenu::new();
    /// menu.push(MenuItem::separator());
    /// ```
    pub fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
            initial: ModifyMenuItem::default(),
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
    /// use winctx::{PopupMenu, MenuItem};
    ///
    /// let mut menu = PopupMenu::new();
    /// menu.push(MenuItem::entry("Example Application"));
    /// menu.push(MenuItem::entry("Exit..."));
    /// ```
    pub fn entry<T>(text: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            kind: MenuItemKind::MenyEntry {
                text: text.to_string(),
            },
            initial: ModifyMenuItem::default(),
        }
    }

    /// Set the initial state of the menu item.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::{PopupMenu, MenuItem, ModifyMenuItem};
    ///
    /// let mut menu = PopupMenu::new();
    /// menu.push(MenuItem::entry("Example Application")
    ///    .initial(ModifyMenuItem::new().checked(true)));
    /// ```
    pub fn initial(self, initial: ModifyMenuItem) -> Self {
        Self { initial, ..self }
    }
}
