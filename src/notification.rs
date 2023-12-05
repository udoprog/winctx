use std::fmt;
use std::time::Duration;

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

// Windows-specific implementation details.
#[cfg(target_os = "windows")]
impl NotificationIcon {
    /// Convert into a flag.
    pub(super) fn into_flags(self) -> winapi::shared::minwindef::DWORD {
        use self::NotificationIcon::*;
        use winapi::um::shellapi;

        match self {
            Info => shellapi::NIIF_INFO,
            Error => shellapi::NIIF_ERROR,
            Warning => shellapi::NIIF_WARNING,
        }
    }
}

/// A single notification.
pub struct Notification {
    pub(super) message: String,
    pub(super) title: Option<String>,
    pub(super) icon: NotificationIcon,
    pub(super) timeout: Option<Duration>,
}

impl fmt::Debug for Notification {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Notification")
            .field("message", &self.message)
            .field("title", &self.title)
            .field("icon", &self.icon)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl Notification {
    /// Create a new notification.
    pub fn new<M>(message: M) -> Self
    where
        M: fmt::Display,
    {
        Self {
            message: message.to_string(),
            title: None,
            icon: NotificationIcon::Info,
            timeout: Some(Duration::from_secs(1)),
        }
    }

    /// Set the message for the notification.
    pub fn with_title<T>(self, title: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    /// Set the notification icon.
    pub fn with_icon(self, icon: NotificationIcon) -> Self {
        Self { icon, ..self }
    }
}
