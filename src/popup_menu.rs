use std::fmt;

use crate::event::{MouseButton, MouseButtons};
use crate::menu_item::MenuItemKind;
use crate::{AreaId, ItemId, MenuItem};

/// The structure of a popup menu.
pub struct PopupMenu {
    area_id: AreaId,
    pub(super) menu: Vec<MenuItem>,
    /// The default item in the menu.
    pub(super) default: Option<u32>,
    /// Mouse buttons which will be accepted to open the menu.
    pub(super) open_menu: MouseButtons,
}

impl PopupMenu {
    /// Construct a new empt popup menu.
    pub(super) fn new(area_id: AreaId) -> Self {
        Self {
            area_id,
            menu: Vec::new(),
            default: None,
            open_menu: MouseButtons::RIGHT,
        }
    }

    /// Specify a collection of mouse buttons which will be accepted to open the
    /// context menu.
    ///
    /// By default this is [`MouseButton::Right`].
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    /// use winctx::event::MouseButton;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");;
    /// let area = window.new_area();
    ///
    /// let menu = area.popup_menu().open_menu([MouseButton::Left, MouseButton::Right]);
    /// menu.push_entry("Example Application");
    /// menu.push_separator();
    /// menu.push_entry("Exit...");
    /// ```
    pub fn open_menu<I>(self, buttons: I) -> Self
    where
        I: IntoIterator<Item = MouseButton>,
    {
        Self {
            open_menu: MouseButtons::from_iter(buttons),
            ..self
        }
    }

    /// Construct a menu entry.
    ///
    /// The `default` parameter indicates whether the entry shoudl be
    /// highlighted.
    ///
    /// This returns a token which can be matched against the token returned in
    /// [`Event::MenuItemClicked`].
    ///
    /// [`Event::MenuItemClicked`]: crate::Event::MenuItemClicked
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");;
    /// let area = window.new_area();
    ///
    /// let menu = area.popup_menu();
    /// menu.push_entry("Example Application");
    /// menu.push_separator();
    /// menu.push_entry("Exit...");
    /// ```
    pub fn push_entry<T>(&mut self, text: T) -> &mut MenuItem
    where
        T: fmt::Display,
    {
        let menu_id = ItemId::new(self.area_id.id(), self.menu.len() as u32);
        self.menu.push(MenuItem::new(
            menu_id,
            MenuItemKind::String {
                text: text.to_string(),
            },
        ));
        self.menu.last_mut().unwrap()
    }

    /// Construct a menu separator.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::WindowBuilder;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");;
    /// let area = window.new_area();
    ///
    /// let menu = area.popup_menu();
    /// menu.push_separator();
    /// ```
    pub fn push_separator(&mut self) -> &mut MenuItem {
        let menu_id = ItemId::new(self.area_id.id(), self.menu.len() as u32);
        self.menu
            .push(MenuItem::new(menu_id, MenuItemKind::Separator));
        self.menu.last_mut().unwrap()
    }

    /// Set the default item in the menu.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::WindowBuilder;
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");
    /// let area = window.new_area();
    ///
    /// let menu = area.popup_menu();
    /// let first = menu.push_entry("Example Application").id();
    /// menu.push_separator();
    /// menu.push_entry("Exit...");
    /// menu.set_default(first);
    /// ```
    pub fn set_default(&mut self, menu_item_id: ItemId) {
        if self.area_id == menu_item_id.area_id() {
            self.default = Some(menu_item_id.id());
        }
    }
}
