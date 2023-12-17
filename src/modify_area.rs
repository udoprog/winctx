use std::fmt;

use crate::Icon;

/// A message sent to modify a notification area.
#[derive(Default, Debug)]
pub struct ModifyArea {
    pub(super) icon: Option<Icon>,
    pub(super) tooltip: Option<Box<str>>,
}

impl ModifyArea {
    /// Construct a new empty modification.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the icon of the notification area.
    pub fn icon(self, icon: Icon) -> Self {
        Self {
            icon: Some(icon),
            ..self
        }
    }

    /// Set the tooltip of the notification area.
    pub fn tooltip<T>(self, tooltip: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            tooltip: Some(tooltip.to_string().into()),
            ..self
        }
    }
}
