//! Types related to menu construction.

use std::fmt;

pub(crate) enum MenuItemKind {
    Separator,
    MenyEntry { text: String, default: bool },
}

/// A menu item in the context menu.
///
/// Constructed through [`Builder`].
pub struct MenuItem {
    pub(crate) kind: MenuItemKind,
}

impl MenuItem {
    /// Construct a menu separator.
    pub fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
        }
    }

    /// Construct a menu entry.
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
