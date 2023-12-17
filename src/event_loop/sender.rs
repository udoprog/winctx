use std::fmt;

use tokio::sync::mpsc;

use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use crate::{AreaId, Icon, Notification, Token};

#[derive(Debug)]
pub(crate) enum InputEvent {
    Shutdown,
    ClearTooltip(AreaId),
    SetTooltip(AreaId, String),
    SetIcon(AreaId, Icon),
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

    /// Set the icon of the context menu.
    pub fn set_icon(&self, area_id: AreaId, icon: Icon) {
        _ = self.inner.tx.send(InputEvent::SetIcon(area_id, icon));
    }

    /// Clear the tooltip of the context menu.
    pub fn clear_tooltip(&self, area_id: AreaId) {
        _ = self.inner.tx.send(InputEvent::ClearTooltip(area_id));
    }

    /// Set the tooltip of the context menu.
    pub fn set_tooltip<E>(&self, area_id: AreaId, message: E)
    where
        E: fmt::Display,
    {
        _ = self
            .inner
            .tx
            .send(InputEvent::SetTooltip(area_id, message.to_string()));
    }

    /// Send the given notification.
    pub fn notification(&self, area_id: AreaId, n: Notification) -> Token {
        let id = self
            .inner
            .notifications
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        _ = self.inner.tx.send(InputEvent::Notification(area_id, id, n));
        Token::new(id)
    }

    /// Cause the window to shut down.
    pub fn shutdown(&self) {
        _ = self.inner.tx.send(InputEvent::Shutdown);
    }
}
