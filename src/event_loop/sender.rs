use std::fmt;

use tokio::sync::mpsc;

use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use crate::{Notification, Token};

#[derive(Debug)]
pub(crate) enum InputEvent {
    Shutdown,
    Cleared,
    Errored(String),
    Notification(u32, Notification),
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

    /// Reset the current state to defaults.
    ///
    /// This will clear any error state previously set.
    pub fn clear(&self) {
        _ = self.inner.tx.send(InputEvent::Cleared);
    }

    /// Set an error with the given message.
    ///
    /// The message will be used as the tooltip when the icon is hovered.
    pub fn error<E>(&self, error: E)
    where
        E: fmt::Display,
    {
        _ = self.inner.tx.send(InputEvent::Errored(error.to_string()));
    }

    /// Send the given notification.
    pub fn notification(&self, n: Notification) -> Token {
        let id = self
            .inner
            .notifications
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        _ = self.inner.tx.send(InputEvent::Notification(id, n));
        Token::new(id)
    }

    /// Cause the window to shut down.
    pub fn shutdown(&self) {
        _ = self.inner.tx.send(InputEvent::Shutdown);
    }
}
