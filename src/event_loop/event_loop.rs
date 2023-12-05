use tokio::sync::mpsc;

use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::window::{Window, WindowEvent};
use crate::Result;

use super::{Event, Icon, InputEvent, Token};

/// The event loop being run.
pub struct EventLoop {
    icon: Option<Icon>,
    icon_error: Option<Icon>,
    events_rx: mpsc::UnboundedReceiver<InputEvent>,
    window: Window,
    balloons: Vec<u32>,
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
            balloons: Vec::new(),
        }
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
                            self.balloons.push(id);
                            self.window.send_notification(n).map_err(SendNotification)?;
                        }
                        InputEvent::Shutdown => {
                            println!("Joining window...");
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
                            if let Some(token) = self.balloons.pop() {
                                return Ok(Event::NotificationClicked(Token::new(token)));
                            }
                        }
                        WindowEvent::BalloonTimeout => {
                            if let Some(token) = self.balloons.pop() {
                                return Ok(Event::NotificationTimeout(Token::new(token)));
                            }
                        }
                    }
                }
            }
        }
    }
}
