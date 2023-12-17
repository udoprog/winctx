use std::ffi::OsStr;
use std::ffi::OsString;

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
    class_name: OsString,
    window_name: Option<OsString>,
    menu: Vec<MenuItem>,
    icon: Option<Icon>,
    error_icon: Option<Icon>,
    clipboard_events: bool,
}

impl ContextBuilder {
    /// Construct a new event loop where the window has the specified class
    /// name.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::ContextBuilder;
    ///
    /// let mut builder = ContextBuilder::new("Example Application");
    /// ```
    pub fn new<N>(class_name: N) -> Self
    where
        N: AsRef<OsStr>,
    {
        Self {
            class_name: class_name.as_ref().to_owned(),
            window_name: None,
            menu: Vec::new(),
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

    /// Modify the window name for use in the application.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::ContextBuilder;
    ///
    /// let mut builder = ContextBuilder::new("se.tedro.Example")
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

        let mut window = WindowLoop::new(
            &self.class_name,
            self.window_name.as_deref(),
            self.clipboard_events,
            !self.menu.is_empty(),
        )
        .await
        .map_err(WindowSetup)?;

        self.setup_menu(&mut window).map_err(SetupMenu)?;

        let event_loop = EventLoop::new(events_rx, window);
        let system = Sender::new(events_tx);
        Ok((system, event_loop))
    }

    fn setup_menu(&self, l: &mut WindowLoop) -> Result<(), SetupMenuError> {
        let Some(menu) = &l.menu else {
            return Ok(());
        };

        let icon = match &self.icon {
            Some(icon) => Some(
                window_loop::Icon::from_buffer(&icon.buffer, icon.width, icon.height)
                    .map_err(SetupMenuError::BuildIcon)?,
            ),
            None => None,
        };

        let error_icon = match &self.error_icon {
            Some(icon) => Some(
                window_loop::Icon::from_buffer(&icon.buffer, icon.width, icon.height)
                    .map_err(SetupMenuError::BuildErrorIcon)?,
            ),
            None => None,
        };

        l.window.add_icon().map_err(SetupMenuError::AddIcon)?;

        if let Some(icon) = &icon {
            l.window
                .set_icon(icon.clone())
                .map_err(SetupMenuError::SetIcon)?;
        }

        for (index, item) in self.menu.iter().enumerate() {
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

        l.icons = window_loop::Icons { icon, error_icon };
        Ok(())
    }
}
