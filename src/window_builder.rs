use std::ffi::OsStr;
use std::ffi::OsString;

use tokio::sync::mpsc;

use crate::error::ErrorKind::*;
use crate::error::{SetupIconsError, SetupMenuError};
use crate::menu_item::MenuItemKind;
use crate::window_loop::{IconHandle, MenuHandle, WindowLoop};
use crate::{EventLoop, Icons, MenuId, NotificationMenu, Result, Sender};

/// The builder of a window context.
pub struct WindowBuilder {
    class_name: OsString,
    window_name: Option<OsString>,
    notification_menus: Vec<NotificationMenu>,
    clipboard_events: bool,
    icons: Icons,
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
            notification_menus: Vec::new(),
            clipboard_events: false,
            icons: Icons::default(),
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

    /// Push a notification menu associated with the current application.
    pub fn push_notification_menu(&mut self, notification_menu: NotificationMenu) -> MenuId {
        let id = MenuId::new(self.notification_menus.len() as u32);
        self.notification_menus.push(notification_menu);
        id
    }

    /// Associate custom icons with the window.
    pub fn icons(self, icons: Icons) -> Self {
        Self { icons, ..self }
    }

    /// Construct a new event loop and system integration.
    pub async fn build(self) -> Result<(Sender, EventLoop)> {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let icons = self.setup_icons(&self.icons).map_err(SetupIcons)?;
        let mut menus = Vec::with_capacity(self.notification_menus.len());

        for (id, m) in self.notification_menus.iter().enumerate() {
            let initial_icon = m.initial_icon.map(|i| i.as_usize());
            let menu = MenuHandle::new(MenuId::new(id as u32), initial_icon).map_err(BuildMenu)?;
            self.build_menu(&menu, m).map_err(SetupMenu)?;
            menus.push(menu);
        }

        let mut window = WindowLoop::new(
            &self.class_name,
            self.window_name.as_deref(),
            self.clipboard_events,
            menus,
        )
        .await
        .map_err(WindowSetup)?;

        for menu in &window.menus {
            window
                .window
                .add_notification(menu.menu_id)
                .map_err(AddIcon)?;

            if let Some(icon) = menu.initial_icon {
                window
                    .window
                    .set_icon(menu.menu_id, &icons[icon])
                    .map_err(SetIcon)?;
            }
        }

        let event_loop = EventLoop::new(events_rx, window, icons);
        let system = Sender::new(events_tx);
        Ok((system, event_loop))
    }

    fn setup_icons(&self, icons: &Icons) -> Result<Vec<IconHandle>, SetupIconsError> {
        let mut handles = Vec::with_capacity(icons.icons.len());

        for icon in icons.icons.iter() {
            handles.push(
                IconHandle::from_buffer(icon.as_bytes(), icon.width(), icon.height())
                    .map_err(SetupIconsError::BuildIcon)?,
            );
        }

        Ok(handles)
    }

    fn build_menu(
        &self,
        menu: &MenuHandle,
        notification_menu: &NotificationMenu,
    ) -> Result<(), SetupMenuError> {
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
