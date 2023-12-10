use std::fmt;

use tokio::sync::mpsc;

use crate::error::ErrorKind::*;
use crate::error::SetupMenuError;
use crate::window::Window;
use crate::Result;

use super::{EventLoop, Sender, Token};

pub(super) struct Icon {
    pub(super) buffer: Box<[u8]>,
    pub(super) width: u32,
    pub(super) height: u32,
}

pub enum MenuItem {
    Separator,
    MenyEntry { text: String, default: bool },
}

/// An event loop builder.
pub struct WindowBuilder {
    name: String,
    menu: Vec<MenuItem>,
    class_name: Option<String>,
    icon: Option<Icon>,
    error_icon: Option<Icon>,
}

impl WindowBuilder {
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
        }
    }

    /// Modify the class name to use for the application.
    pub fn with_class_name<C>(self, class_name: C) -> Self
    where
        C: fmt::Display,
    {
        Self {
            class_name: Some(class_name.to_string()),
            ..self
        }
    }

    /// Add a context menu separator.
    pub fn add_menu_separator(&mut self) -> Token {
        let token = Token::new(self.menu.len() as u32);
        self.menu.push(MenuItem::Separator);
        token
    }

    /// Add a context menu separator.
    pub fn add_menu_entry<T>(&mut self, text: T, default: bool) -> Token
    where
        T: fmt::Display,
    {
        let token = Token::new(self.menu.len() as u32);

        self.menu.push(MenuItem::MenyEntry {
            text: text.to_string(),
            default,
        });

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

        let mut window = Window::new(class_name, &self.name)
            .await
            .map_err(WindowSetup)?;

        if let Some(icon) = &self.icon {
            window
                .set_icon_from_buffer(&icon.buffer, icon.width, icon.height)
                .map_err(SetIcon)?;
        }

        self.setup_menu(&mut window).map_err(SetupMenu)?;

        let event_loop = EventLoop::new(self.icon, self.error_icon, events_rx, window);
        let system = Sender::new(events_tx);
        Ok((system, event_loop))
    }

    fn setup_menu(&self, window: &mut Window) -> Result<(), SetupMenuError> {
        for (index, item) in self.menu.iter().enumerate() {
            debug_assert!(u32::try_from(index).is_ok());

            match item {
                MenuItem::Separator => {
                    window
                        .add_menu_separator(index as u32)
                        .map_err(|e| SetupMenuError::AddMenuSeparator(index, e))?;
                }
                MenuItem::MenyEntry { text, default } => {
                    window
                        .add_menu_entry(index as u32, text.as_str(), *default)
                        .map_err(|e| SetupMenuError::AddMenuEntry(index, e))?;
                }
            }
        }

        Ok(())
    }
}
