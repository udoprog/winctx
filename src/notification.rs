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
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the message for the notification.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    ///
    /// let notification = Notification::new()
    ///     .message("This is a body");
    /// ```
    pub fn message<M>(self, message: M) -> Self
    where
        M: fmt::Display,
    {
        Self {
            message: Some(message.to_string()),
            ..self
        }
    }

    /// Set the message for the notification.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    ///
    /// let notification = Notification::new()
    ///     .title("This is a title")
    ///     .message("This is a body");
    /// ```
    pub fn title<M>(self, title: M) -> Self
    where
        M: fmt::Display,
    {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    /// Set the notification icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    /// use winctx::notification::NotificationIcon;
    ///
    /// let notification = Notification::new()
    ///     .message("Something dangerous")
    ///     .icon(NotificationIcon::Warning);
    /// ```
    pub fn icon(self, icon: NotificationIcon) -> Self {
        Self { icon, ..self }
    }

    /// Do not play the sound associated with a notification.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    /// use winctx::notification::NotificationIcon;
    ///
    /// let notification = Notification::new()
    ///     .message("Something dangerous")
    ///     .icon(NotificationIcon::Warning)
    ///     .no_sound();
    /// ```
    pub fn no_sound(self) -> Self {
        Self {
            options: self.options | NIIF_NOSOUND,
            ..self
        }
    }

    /// The large version of the icon should be used as the notification icon.
    ///
    /// Note that this is a hint and might only have an effect in certain
    /// contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    /// use winctx::notification::NotificationIcon;
    ///
    /// let notification = Notification::new()
    ///     .icon(NotificationIcon::Warning)
    ///     .large_icon();
    /// ```
    pub fn large_icon(self) -> Self {
        Self {
            options: self.options | NIIF_LARGE_ICON,
            ..self
        }
    }

    /// The notification should not be presented if the user is in "quiet time".
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::Notification;
    /// use winctx::notification::NotificationIcon;
    ///
    /// let notification = Notification::new()
    ///     .message("Something dangerous")
    ///     .icon(NotificationIcon::Warning)
    ///     .respect_quiet_time();
    /// ```
    pub fn respect_quiet_time(self) -> Self {
        Self {
            options: self.options | NIIF_RESPECT_QUIET_TIME,
            ..self
        }
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

impl Default for Notification {
    #[inline]
    fn default() -> Self {
        Self {
            message: None,
            title: None,
            icon: NotificationIcon::Info,
            timeout: Some(Duration::from_secs(1)),
            options: 0,
        }
    }
}
