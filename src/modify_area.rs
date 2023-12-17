use std::fmt;

use crate::Icon;

/// A message sent to modify a notification area.
#[derive(Default, Debug)]
pub(crate) struct ModifyArea {
    pub(super) icon: Option<Icon>,
    pub(super) tooltip: Option<Box<str>>,
}

impl ModifyArea {
    /// Set the icon of the notification area.
    pub(crate) fn icon(&mut self, icon: Icon) {
        self.icon = Some(icon);
    }

    /// Set the tooltip of the notification area.
    pub(crate) fn tooltip<T>(&mut self, tooltip: T)
    where
        T: fmt::Display,
    {
        self.tooltip = Some(tooltip.to_string().into());
    }
}
