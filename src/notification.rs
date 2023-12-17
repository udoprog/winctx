//! Types related to notifications.

use std::fmt;
use std::time::Duration;

use windows_sys::Win32::UI::Shell::{NIIF_LARGE_ICON, NIIF_NOSOUND, NIIF_RESPECT_QUIET_TIME};

/// Indicates the [standard icon] that Windows should use for the notification.
///
/// [standard icon]: https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-std-icons
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum NotificationIcon {
    /// An information icon.
    Info,
    /// A warning icon.
    Warning,
    /// An error icon.
    Error,
}

/// A single notification.
pub struct Notification {
    pub(super) title: Option<String>,
    pub(super) message: Option<String>,
    pub(super) icon: NotificationIcon,
    pub(super) timeout: Option<Duration>,
    pub(super) options: u32,
}

impl Notification {
    /// Create a new notification.
    pub(super) fn new() -> Self {
        Self {
            message: None,
            title: None,
            icon: NotificationIcon::Info,
            timeout: Some(Duration::from_secs(1)),
            options: 0,
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
        self.icon = icon;
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
}

impl fmt::Debug for Notification {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Notification")
            .field("title", &self.title)
            .field("message", &self.message)
            .field("icon", &self.icon)
            .field("timeout", &self.timeout)
            .finish()
    }
}
