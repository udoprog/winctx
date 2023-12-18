//! Types related to notifications.

use std::fmt;
use std::time::Duration;

use windows_sys::Win32::UI::Shell::{self, NIIF_LARGE_ICON, NIIF_NOSOUND, NIIF_RESPECT_QUIET_TIME};

use crate::icon::StockIcon;

/// Indicates the [standard icon] that Windows should use for the notification.
///
/// [standard icon]: https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-std-icons
#[derive(Debug)]
#[non_exhaustive]
pub(super) enum NotificationIcon {
    /// An information icon.
    Info,
    /// A warning icon.
    Warning,
    /// An error icon.
    Error,
    /// A stock icon icon.
    StockIcon(StockIcon),
}

/// A single notification.
#[derive(Debug)]
pub(super) struct Notification {
    pub(super) title: Option<String>,
    pub(super) message: Option<String>,
    pub(super) icon: Option<NotificationIcon>,
    pub(super) timeout: Option<Duration>,
    pub(super) options: u32,
    pub(super) stock_icon_opts: u32,
}

impl Notification {
    /// Create a new notification.
    pub(super) fn new() -> Self {
        Self {
            message: None,
            title: None,
            icon: None,
            timeout: Some(Duration::from_secs(1)),
            options: 0,
            stock_icon_opts: 0,
        }
    }

    pub(super) fn message<M>(&mut self, message: M)
    where
        M: fmt::Display,
    {
        self.message = Some(message.to_string());
    }

    pub(super) fn title<M>(&mut self, title: M)
    where
        M: fmt::Display,
    {
        self.title = Some(title.to_string());
    }

    pub(super) fn icon(&mut self, icon: NotificationIcon) {
        self.icon = Some(icon);
    }

    pub(super) fn no_sound(&mut self) {
        self.options |= NIIF_NOSOUND;
    }

    pub(super) fn large_icon(&mut self) {
        self.options |= NIIF_LARGE_ICON;
    }

    pub(super) fn respect_quiet_time(&mut self) {
        self.options |= NIIF_RESPECT_QUIET_TIME;
    }

    pub(crate) fn icon_selected(&mut self) {
        self.stock_icon_opts |= Shell::SHGSI_SELECTED;
    }

    pub(crate) fn icon_link_overlay(&mut self) {
        self.stock_icon_opts |= Shell::SHGSI_LINKOVERLAY;
    }
}
