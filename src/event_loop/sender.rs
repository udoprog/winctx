use tokio::sync::mpsc;

use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use crate::{AreaId, ModifyArea, Notification, Token};

#[derive(Debug)]
pub(crate) enum InputEvent {
    Shutdown,
    ModifyArea(AreaId, ModifyArea),
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

    /// Set the tooltip of the context menu.
    pub fn modify_area(&self, area_id: AreaId, modify_area: ModifyArea) {
        _ = self
            .inner
            .tx
            .send(InputEvent::ModifyArea(area_id, modify_area));
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
