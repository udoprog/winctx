use std::fmt;

use tokio::sync::mpsc;

use crate::error::ErrorKind::*;
use crate::error::SetupMenuError;
use crate::menu_item::{MenuItem, MenuItemKind};
use crate::window_loop;
use crate::window_loop::WindowLoop;
use crate::Result;
use crate::{EventLoop, Sender, Token};

pub(super) struct Icon {
    pub(super) buffer: Box<[u8]>,
    pub(super) width: u32,
    pub(super) height: u32,
}

/// The builder of a window context.
pub struct ContextBuilder {
    name: String,
    menu: Vec<MenuItem>,
    class_name: Option<String>,
    icon: Option<Icon>,
    error_icon: Option<Icon>,
    clipboard_events: bool,
}

impl ContextBuilder {
    /// Construct a new event loop where the application has the specified name.
    pub fn new<N>(name: N) -> Self
    where
        N: fmt::Display,
    {
        Self {
            name: name.to_string(),
            menu: Vec::new(),
            class_name: None,
            icon: None,
            error_icon: None,
            clipboard_events: false,
        }
    }

    /// Indicates whether we should monitor the system clipboard for changes.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::ContextBuilder;
    ///
    /// let mut builder = ContextBuilder::new("Example Application")
    ///     .clipboard_events(true);
    /// ```
    pub fn clipboard_events(self, clipboard_events: bool) -> Self {
        Self {
            clipboard_events,
            ..self
        }
    }

    /// Modify the class name to use for the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::ContextBuilder;
    ///
    /// let mut builder = ContextBuilder::new("Example Application")
    ///     .class_name("se.tedro.Example");
    /// ```
    pub fn class_name<C>(self, class_name: C) -> Self
    where
        C: fmt::Display,
    {
        Self {
            class_name: Some(class_name.to_string()),
            ..self
        }
    }

    /// Add a new menu item.
    pub fn push_menu_item(&mut self, menu_item: MenuItem) -> Token {
        let token = Token::new(self.menu.len() as u32);
        self.menu.push(menu_item);
        token
    }

    /// Set the default icon to use for the context menu.
    pub fn set_icon(&mut self, icon: &[u8], width: u32, height: u32) {
        self.icon = Some(Icon {
            buffer: icon.into(),
            width,
            height,
        });
    }

    /// Set the icon to use when an error has been emitted.
    ///
    /// This is the icon that will be set when [`Sender::error`] is used.
    pub fn set_error_icon(&mut self, icon: &[u8], width: u32, height: u32) {
        self.error_icon = Some(Icon {
            buffer: icon.into(),
            width,
            height,
        });
    }

    /// Construct a new event loop and system integration.
    pub async fn build(self) -> Result<(Sender, EventLoop)> {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let class_name = self.class_name.as_deref().unwrap_or(self.name.as_str());

        let mut window = WindowLoop::new(class_name, &self.name, self.clipboard_events)
            .await
            .map_err(WindowSetup)?;

        self.setup_menu(&mut window).map_err(SetupMenu)?;

        let icon = match self.icon {
            Some(icon) => Some(
                window_loop::Icon::from_buffer(&icon.buffer, icon.width, icon.height)
                    .map_err(BuildIcon)?,
            ),
            None => None,
        };

        if let Some(icon) = &icon {
            window.set_icon(icon.clone()).map_err(SetIcon)?;
        }

        let error_icon = match self.error_icon {
            Some(icon) => Some(
                window_loop::Icon::from_buffer(&icon.buffer, icon.width, icon.height)
                    .map_err(BuildErrorIcon)?,
            ),
            None => None,
        };

        let event_loop = EventLoop::new(icon, error_icon, events_rx, window);
        let system = Sender::new(events_tx);
        Ok((system, event_loop))
    }

    fn setup_menu(&self, window: &mut WindowLoop) -> Result<(), SetupMenuError> {
        for (index, item) in self.menu.iter().enumerate() {
            debug_assert!(u32::try_from(index).is_ok());

            match &item.kind {
                MenuItemKind::Separator => {
                    window
                        .add_menu_separator(index as u32)
                        .map_err(|e| SetupMenuError::AddMenuSeparator(index, e))?;
                }
                MenuItemKind::MenyEntry { text, default } => {
                    window
                        .add_menu_entry(index as u32, text.as_str(), *default)
                        .map_err(|e| SetupMenuError::AddMenuEntry(index, e))?;
                }
            }
        }

        Ok(())
    }
}
