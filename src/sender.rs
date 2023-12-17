//! Types related to modifying the window context.

use std::fmt;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{AreaId, Icon, ItemId, ModifyArea, ModifyMenuItem, Notification, NotificationId};

#[derive(Debug)]
pub(crate) enum InputEvent {
    Shutdown,
    ModifyArea(AreaId, ModifyArea),
    ModifyMenuItem {
        item_id: ItemId,
        modify: ModifyMenuItem,
    },
    Notification(AreaId, u32, Notification),
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
    pub fn notification(&self, area_id: AreaId, n: Notification) -> NotificationId {
        let id = self
            .inner
            .notifications
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        _ = self.inner.tx.send(InputEvent::Notification(area_id, id, n));
        NotificationId::new(id)
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
    pub fn icon(mut self, icon: Icon) -> Self {
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
        _ = self
            .tx
            .send(InputEvent::ModifyArea(self.area_id, self.modify));
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
