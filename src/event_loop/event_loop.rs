use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::window::{Window, WindowEvent};
use crate::Notification;
use crate::Result;

use super::{Event, Icon, InputEvent, Token};

/// The event loop being run.
pub struct EventLoop {
    icon: Option<Icon>,
    icon_error: Option<Icon>,
    events_rx: mpsc::UnboundedReceiver<InputEvent>,
    window: Window,
    visible: Option<u32>,
    pending: VecDeque<(u32, Notification)>,
}

impl EventLoop {
    pub(super) fn new(
        icon: Option<Icon>,
        icon_error: Option<Icon>,
        events_rx: mpsc::UnboundedReceiver<InputEvent>,
        window: Window,
    ) -> Self {
        Self {
            icon,
            icon_error,
            events_rx,
            window,
            visible: None,
            pending: VecDeque::new(),
        }
    }

    fn take_notification(&mut self) -> Result<u32> {
        let id = self.visible.take().ok_or(MissingNotification)?;

        if let Some((id, n)) = self.pending.pop_front() {
            self.visible = Some(id);
            self.window
                .send_notification(id, n)
                .map_err(SendNotification)?;
        }

        Ok(id)
    }

    /// Tick the event loop.
    pub async fn tick(&mut self) -> Result<Event> {
        if self.window.is_closed() {
            return Err(Error::new(WindowClosed));
        };

        loop {
            tokio::select! {
                Some(event) = self.events_rx.recv() => {
                    match event {
                        InputEvent::Cleared => {
                            self.window.set_tooltip("").map_err(SetTooltip)?;

                            if let Some(icon) = &self.icon {
                                self.window.set_icon_from_buffer(&icon.buffer, icon.width, icon.height).map_err(SetIcon)?;
                            }
                        }
                        InputEvent::Errored(message) => {
                            self.window.set_tooltip(&message).map_err(SetTooltip)?;

                            if let Some(icon) = &self.icon_error {
                                self.window.set_icon_from_buffer(&icon.buffer, icon.width, icon.height).map_err(SetIcon)?;
                            }
                        }
                        InputEvent::Notification(id, n) => {
                            if self.visible.is_some() {
                                self.pending.push_back((id, n));
                            } else {
                                self.visible = Some(id);
                                self.window.send_notification(id, n).map_err(SendNotification)?;
                            }
                        }
                        InputEvent::Shutdown => {
                            self.window.join()?;
                            return Ok(Event::Shutdown);
                        }
                    }
                }
                e = self.window.tick() => {
                    match e {
                        WindowEvent::MenuClicked(idx) => {
                            return Ok(Event::MenuEntryClicked(Token::new(idx)));
                        },
                        WindowEvent::Shutdown => {
                            self.window.join()?;
                            return Ok(Event::Shutdown);
                        }
                        WindowEvent::BalloonClicked => {
                            let current = self.take_notification()?;
                            return Ok(Event::NotificationClicked(Token::new(current)));
                        }
                        WindowEvent::BalloonTimeout => {
                            let current = self.take_notification()?;
                            return Ok(Event::NotificationDismissed(Token::new(current)));
                        }
                    }
                }
            }
        }
    }
}
