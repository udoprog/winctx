//! Types related to modifying the window context.

use std::fmt;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::icon::StockIcon;
use crate::notification::NotificationIcon;
use crate::{AreaId, IconId, ItemId, ModifyArea, ModifyMenuItem, Notification, NotificationId};

#[derive(Debug)]
pub(super) enum InputEvent {
    Shutdown,
    ModifyArea {
        area_id: AreaId,
        modify: ModifyArea,
    },
    ModifyMenuItem {
        item_id: ItemId,
        modify: ModifyMenuItem,
    },
    Notification {
        area_id: AreaId,
        notification_id: NotificationId,
        notification: Notification,
    },
}

struct Inner {
    notifications: AtomicU32,
    tx: mpsc::UnboundedSender<InputEvent>,
}

/// Handle used to interact with the system integration.
#[derive(Clone)]
pub struct Sender {
    inner: Arc<Inner>,
}

impl Sender {
    pub(crate) fn new(tx: mpsc::UnboundedSender<InputEvent>) -> Self {
        Self {
            inner: Arc::new(Inner {
                notifications: AtomicU32::new(0),
                tx,
            }),
        }
    }

    /// Start a modify area request.
    ///
    /// This needs to be send using [`ModifyAreaBuilder::send`] to actually
    /// apply.
    pub fn modify_area(&self, area_id: AreaId) -> ModifyAreaBuilder<'_> {
        ModifyAreaBuilder {
            tx: &self.inner.tx,
            area_id,
            modify: ModifyArea::default(),
        }
    }

    /// Modify a menu item.
    pub fn modify_menu_item(&self, item_id: ItemId) -> ModifyMenuItemBuilder<'_> {
        ModifyMenuItemBuilder {
            tx: &self.inner.tx,
            item_id,
            modify: ModifyMenuItem::default(),
        }
    }

    /// Send the given notification.
    pub fn notification(&self, area_id: AreaId) -> NotificationBuilder<'_> {
        let id = self
            .inner
            .notifications
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        NotificationBuilder {
            tx: &self.inner.tx,
            area_id,
            id: NotificationId::new(id),
            notification: Notification::new(),
        }
    }

    /// Cause the window to shut down.
    pub fn shutdown(&self) {
        _ = self.inner.tx.send(InputEvent::Shutdown);
    }
}

/// A builder returned by [`Sender::modify_area`].
#[must_use = "Must call `send()` to apply changes"]
pub struct ModifyAreaBuilder<'a> {
    tx: &'a mpsc::UnboundedSender<InputEvent>,
    area_id: AreaId,
    modify: ModifyArea,
}

impl ModifyAreaBuilder<'_> {
    /// Set the icon of the notification area.
    pub fn icon(mut self, icon: IconId) -> Self {
        self.modify.icon(icon);
        self
    }

    /// Set the tooltip of the notification area.
    pub fn tooltip<T>(mut self, tooltip: T) -> Self
    where
        T: fmt::Display,
    {
        self.modify.tooltip(tooltip);
        self
    }

    /// Send the modification.
    pub fn send(self) {
        _ = self.tx.send(InputEvent::ModifyArea {
            area_id: self.area_id,
            modify: self.modify,
        });
    }
}

/// A builder returned by [`Sender::modify_menu_item`].
#[must_use = "Must call `send()` to apply changes"]
pub struct ModifyMenuItemBuilder<'a> {
    tx: &'a mpsc::UnboundedSender<InputEvent>,
    item_id: ItemId,
    modify: ModifyMenuItem,
}

impl ModifyMenuItemBuilder<'_> {
    /// Set the checked state of the menu item.
    pub fn checked(mut self, checked: bool) -> Self {
        self.modify.checked(checked);
        self
    }

    /// Set that the menu item should be highlighted.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.modify.highlight(highlight);
        self
    }

    /// Send the modification.
    pub fn send(self) {
        _ = self.tx.send(InputEvent::ModifyMenuItem {
            item_id: self.item_id,
            modify: self.modify,
        });
    }
}

/// A builder returned by [`Sender::notification`].
#[must_use = "Must call `send()` to send the notification"]
pub struct NotificationBuilder<'a> {
    tx: &'a mpsc::UnboundedSender<InputEvent>,
    area_id: AreaId,
    id: NotificationId,
    notification: Notification,
}

impl NotificationBuilder<'_> {
    /// Set the message for the notification.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .message("This is a body")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn message<M>(mut self, message: M) -> Self
    where
        M: fmt::Display,
    {
        self.notification.message(message);
        self
    }

    /// Set the message for the notification.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .title("This is a title")
    ///     .message("This is a body")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn title<M>(mut self, title: M) -> Self
    where
        M: fmt::Display,
    {
        self.notification.title(title);
        self
    }

    /// Set the notification to be informational.
    ///
    /// This among other things causes the icon to indicate that it's
    /// informational.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .info()
    ///     .message("Something normal")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn info(mut self) -> Self {
        self.notification.icon(NotificationIcon::Info);
        self
    }

    /// Set the notification to be a warning.
    ///
    /// This among other things causes the icon to indicate a warning.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something strange")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn warning(mut self) -> Self {
        self.notification.icon(NotificationIcon::Warning);
        self
    }

    /// Set the notification to be an error.
    ///
    /// This among other things causes the icon to indicate an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .error()
    ///     .message("Something broken")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn error(mut self) -> Self {
        self.notification.icon(NotificationIcon::Error);
        self
    }

    /// Set a stock icon for the notification.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    /// use winctx::icon::StockIcon;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .error()
    ///     .message("Something broken")
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn stock_icon(mut self, stock_icon: StockIcon) -> Self {
        self.notification
            .icon(NotificationIcon::StockIcon(stock_icon));
        self
    }

    /// Do not play the sound associated with a notification.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something dangerous")
    ///     .no_sound()
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn no_sound(mut self) -> Self {
        self.notification.no_sound();
        self
    }

    /// The large version of the icon should be used as the notification icon.
    ///
    /// Note that this is a hint and might only have an effect in certain
    /// contexts.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something dangerous")
    ///     .large_icon()
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn large_icon(mut self) -> Self {
        self.notification.large_icon();
        self
    }

    /// Indicates that the icon should be highlighted for selection.
    ///
    /// Note that this only has an effect on certain icons:
    /// * Stock icons specified with [`NotificationBuilder::stock_icon`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    /// use winctx::icon::StockIcon;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something dangerous")
    ///     .stock_icon(StockIcon::FOLDER)
    ///     .icon_selected()
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn icon_selected(mut self) -> Self {
        self.notification.icon_selected();
        self
    }

    /// Indicates that the icon should have the link overlay.
    ///
    /// Note that this only has an effect on certain icons:
    /// * Stock icons specified with [`NotificationBuilder::stock_icon`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    /// use winctx::icon::StockIcon;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something dangerous")
    ///     .stock_icon(StockIcon::FOLDER)
    ///     .icon_link_overlay()
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn icon_link_overlay(mut self) -> Self {
        self.notification.icon_link_overlay();
        self
    }

    /// The notification should not be presented if the user is in "quiet time".
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::CreateWindow;
    ///
    /// # async fn test() -> winctx::Result<()> {
    /// let mut window = CreateWindow::new("se.tedro.Example");;
    /// let area = window.new_area().id();
    ///
    /// let (mut sender, _) = window.build().await?;
    ///
    /// let id = sender.notification(area)
    ///     .warning()
    ///     .message("Something dangerous")
    ///     .respect_quiet_time()
    ///     .send();
    /// # Ok(()) }
    /// ```
    pub fn respect_quiet_time(mut self) -> Self {
        self.notification.respect_quiet_time();
        self
    }

    /// Send the modification and return the identifier of the sent
    /// notification.
    pub fn send(self) -> NotificationId {
        _ = self.tx.send(InputEvent::Notification {
            area_id: self.area_id,
            notification_id: self.id,
            notification: self.notification,
        });
        self.id
    }
}
