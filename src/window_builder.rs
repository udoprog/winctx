use std::ffi::OsStr;
use std::ffi::OsString;

use tokio::sync::mpsc;

use crate::error::ErrorKind::*;
use crate::error::{SetupIconsError, SetupMenuError};
use crate::menu_item::MenuItemKind;
use crate::window_loop::PopupMenuHandle;
use crate::window_loop::{AreaHandle, IconHandle, WindowLoop};
use crate::MenuItemId;
use crate::PopupMenu;
use crate::{Area, AreaId, EventLoop, Icons, Result, Sender};

/// The builder of a window context.
pub struct WindowBuilder {
    class_name: OsString,
    window_name: Option<OsString>,
    areas: Vec<Area>,
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
    /// let mut builder = WindowBuilder::new("se.tedro.Example");
    /// ```
    pub fn new<N>(class_name: N) -> Self
    where
        N: AsRef<OsStr>,
    {
        Self {
            class_name: class_name.as_ref().to_owned(),
            window_name: None,
            areas: Vec::new(),
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
    /// let mut builder = WindowBuilder::new("se.tedro.Example")
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

    /// Push a notification area onto the window and return its id.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::{Area, Icons, WindowBuilder, ModifyArea};
    ///
    /// # macro_rules! include_bytes { ($path:literal) => { &[] } }
    /// const ICON: &[u8] = include_bytes!("tokio.ico");
    ///
    /// let mut icons = Icons::new();
    /// let icon = icons.push_buffer(ICON, 22, 22);
    ///
    /// let mut builder = WindowBuilder::new("se.tedro.Example").icons(icons);
    /// let area = Area::new().initial(ModifyArea::new().icon(icon));
    /// let area_id = builder.push_area(area);
    /// ```
    pub fn push_area(&mut self, area: Area) -> AreaId {
        let id = AreaId::new(self.areas.len() as u32);
        self.areas.push(area);
        id
    }

    /// Associate custom icons with the window.
    ///
    /// If [`Icon`] handles are used their associated icons lexicon has to be
    /// installed here.
    ///
    /// [`Icon`]: crate::Icon
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::{Icons, Area, WindowBuilder, ModifyArea};
    ///
    /// # macro_rules! include_bytes { ($path:literal) => { &[] } }
    /// const ICON: &[u8] = include_bytes!("tokio.ico");
    ///
    /// let mut icons = Icons::new();
    /// let default_icon = icons.push_buffer(ICON, 22, 22);
    /// let area = Area::new()
    ///     .initial(ModifyArea::new().icon(default_icon));
    ///
    /// let mut builder = WindowBuilder::new("se.tedro.Example")
    ///     .icons(icons);
    /// ```
    pub fn icons(self, icons: Icons) -> Self {
        Self { icons, ..self }
    }

    /// Construct a new event loop and system integration.
    pub async fn build(self) -> Result<(Sender, EventLoop)> {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        let icons = self.setup_icons(&self.icons).map_err(SetupIcons)?;
        let mut menus = Vec::with_capacity(self.areas.len());
        let mut initial = Vec::new();

        for (id, m) in self.areas.iter().enumerate() {
            let area_id = AreaId::new(id as u32);

            let popup_menu = if let Some(popup_menu) = &m.popup_menu {
                let mut menu = PopupMenuHandle::new().map_err(BuildPopupMenu)?;
                build_menu(&mut menu, popup_menu).map_err(SetupMenu)?;
                Some(menu)
            } else {
                None
            };

            if let Some(modify) = &m.initial {
                initial.push((area_id, modify));
            }

            menus.push(AreaHandle::new(area_id, popup_menu));
        }

        let mut window = WindowLoop::new(
            &self.class_name,
            self.window_name.as_deref(),
            self.clipboard_events,
            menus,
        )
        .await
        .map_err(WindowSetup)?;

        for menu in &window.areas {
            window
                .window
                .add_notification(menu.area_id)
                .map_err(AddNotification)?;
        }

        for (area_id, modify) in initial {
            let icon = modify.icon.and_then(|icon| icons.get(icon.as_usize()));

            window
                .window
                .modify_notification(area_id, icon, modify.tooltip.as_deref())
                .map_err(ModifyNotification)?;
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
}

fn build_menu(menu: &mut PopupMenuHandle, popup_menu: &PopupMenu) -> Result<(), SetupMenuError> {
    for (index, item) in popup_menu.menu.iter().enumerate() {
        debug_assert!(u32::try_from(index).is_ok());
        let menu_item_id = MenuItemId::new(index as u32);

        match &item.kind {
            MenuItemKind::Separator => {
                let default = popup_menu.default == Some(menu_item_id);

                menu.add_menu_separator(menu_item_id, default, &item.initial)
                    .map_err(|e| SetupMenuError::AddMenuSeparator(index, e))?;
            }
            MenuItemKind::MenyEntry { text } => {
                let default = popup_menu.default == Some(menu_item_id);

                menu.add_menu_entry(menu_item_id, text.as_str(), default, &item.initial)
                    .map_err(|e| SetupMenuError::AddMenuEntry(index, e))?;
            }
        }
    }

    Ok(())
}
