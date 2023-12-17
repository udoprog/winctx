use std::ffi::OsStr;
use std::ffi::OsString;

use tokio::sync::mpsc;

use crate::error::ErrorKind::*;
use crate::error::SetupMenuError;
use crate::menu_item::MenuItemKind;
use crate::window_loop;
use crate::window_loop::WindowLoop;
use crate::NotificationIcons;
use crate::NotificationMenu;
use crate::Result;
use crate::{EventLoop, Icon, Sender};

/// The builder of a window context.
pub struct WindowBuilder {
    class_name: OsString,
    window_name: Option<OsString>,
    notification_menu: Option<NotificationMenu>,
    notification_icons: Option<NotificationIcons>,
    clipboard_events: bool,
    initial_icon: Option<Icon>,
}

impl WindowBuilder {
    /// Construct a new event loop where the window has the specified class
    /// name.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    ///
    /// let mut builder = WindowBuilder::new("Example Application");
    /// ```
    pub fn new<N>(class_name: N) -> Self
    where
        N: AsRef<OsStr>,
    {
        Self {
            class_name: class_name.as_ref().to_owned(),
            window_name: None,
            notification_menu: None,
            notification_icons: None,
            clipboard_events: false,
            initial_icon: None,
        }
    }

    /// Indicates whether we should monitor the system clipboard for changes.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    ///
    /// let mut builder = WindowBuilder::new("Example Application")
    ///     .clipboard_events(true);
    /// ```
    pub fn clipboard_events(self, clipboard_events: bool) -> Self {
        Self {
            clipboard_events,
            ..self
        }
    }

    /// Modify the window name for use in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    ///
    /// let mut builder = WindowBuilder::new("se.tedro.Example")
    ///     .window_name("Example Application");
    /// ```
    pub fn window_name<N>(self, window_name: N) -> Self
    where
        N: AsRef<OsStr>,
    {
        Self {
            window_name: Some(window_name.as_ref().to_owned()),
            ..self
        }
    }

    /// Set the notification menu to use in the tray of the constructed window.
    pub fn notification_menu(self, notification_menu: NotificationMenu) -> Self {
        Self {
            notification_menu: Some(notification_menu),
            ..self
        }
    }

    /// Set the notification icons to use in the tray of the constructed window.
    pub fn notification_icons(self, notification_icons: NotificationIcons) -> Self {
        Self {
            notification_icons: Some(notification_icons),
            ..self
        }
    }

    /// Set the default icon to use.
    pub fn initial_icon(self, icon: Icon) -> Self {
        Self {
            initial_icon: Some(icon),
            ..self
        }
    }

    /// Construct a new event loop and system integration.
    pub async fn build(self) -> Result<(Sender, EventLoop)> {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let mut window = WindowLoop::new(
            &self.class_name,
            self.window_name.as_deref(),
            self.clipboard_events,
            self.notification_menu.is_some(),
        )
        .await
        .map_err(WindowSetup)?;

        self.setup_menu(&mut window).map_err(SetupMenu)?;

        let event_loop = EventLoop::new(events_rx, window);
        let system = Sender::new(events_tx);
        Ok((system, event_loop))
    }

    fn setup_menu(&self, l: &mut WindowLoop) -> Result<(), SetupMenuError> {
        let (Some(menu), Some(notification_menu)) = (&l.menu, &self.notification_menu) else {
            return Ok(());
        };

        if let Some(notification_icons) = &self.notification_icons {
            let mut icons = Vec::with_capacity(notification_icons.icons.len());

            for icon in notification_icons.icons.iter() {
                icons.push(
                    window_loop::IconHandle::from_buffer(
                        icon.as_bytes(),
                        icon.width(),
                        icon.height(),
                    )
                    .map_err(SetupMenuError::BuildIcon)?,
                );
            }

            l.window.add_icon().map_err(SetupMenuError::AddIcon)?;

            if let Some(icon) = self.initial_icon {
                l.window
                    .set_icon(&icons[icon.as_usize()])
                    .map_err(SetupMenuError::SetIcon)?;
            }

            l.icons = icons;
        }

        for (index, item) in notification_menu.menu.iter().enumerate() {
            debug_assert!(u32::try_from(index).is_ok());

            match &item.kind {
                MenuItemKind::Separator => {
                    menu.add_menu_separator(index as u32)
                        .map_err(|e| SetupMenuError::AddMenuSeparator(index, e))?;
                }
                MenuItemKind::MenyEntry { text, default } => {
                    menu.add_menu_entry(index as u32, text.as_str(), *default)
                        .map_err(|e| SetupMenuError::AddMenuEntry(index, e))?;
                }
            }
        }

        Ok(())
    }
}
