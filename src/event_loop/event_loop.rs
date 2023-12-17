use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::token::Token;
use crate::window_loop::IconHandle;
use crate::window_loop::{WindowEvent, WindowLoop};
use crate::Notification;
use crate::Result;

use super::{Event, InputEvent};

/// The event loop being run.
#[repr(C)]
pub struct EventLoop {
    events_rx: mpsc::UnboundedReceiver<InputEvent>,
    window_loop: WindowLoop,
    icons: Vec<IconHandle>,
    visible: Option<u32>,
    pending: VecDeque<(u32, Notification)>,
}

impl EventLoop {
    pub(crate) fn new(
        events_rx: mpsc::UnboundedReceiver<InputEvent>,
        window_loop: WindowLoop,
        icons: Vec<IconHandle>,
    ) -> Self {
        Self {
            events_rx,
            window_loop,
            icons,
            visible: None,
            pending: VecDeque::new(),
        }
    }

    fn take_notification(&mut self) -> Result<u32> {
        let id = self.visible.take().ok_or(MissingNotification)?;

        if let Some((id, n)) = self.pending.pop_front() {
            self.visible = Some(id);

            if let Some(menu) = self.window_loop.menus.get(0) {
                self.window_loop
                    .window
                    .send_notification(menu.id, id, n)
                    .map_err(SendNotification)?;
            }
        }

        Ok(id)
    }

    /// Tick the event loop.
    pub async fn tick(&mut self) -> Result<Event> {
        if self.window_loop.is_closed() {
            return Err(Error::new(WindowClosed));
        };

        loop {
            tokio::select! {
                Some(event) = self.events_rx.recv() => {
                    match event {
                        InputEvent::ClearTooltip => {
                            if let Some(menu) = self.window_loop.menus.get(0) {
                                self.window_loop.window.clear_tooltip(menu.id).map_err(ClearTooltip)?;
                            }
                        }
                        InputEvent::SetTooltip(message) => {
                            if let Some(menu) = self.window_loop.menus.get(0) {
                                self.window_loop.window.set_tooltip(menu.id, &message).map_err(SetTooltip)?;
                            }
                        }
                        InputEvent::SetIcon(icon) => {
                            if let Some(menu) = self.window_loop.menus.get(0) {
                                if let Some(icon) = self.icons.get(icon.as_usize()) {
                                    self.window_loop.window.set_icon(menu.id, icon).map_err(SetIcon)?;
                                }
                            }
                        }
                        InputEvent::Notification(id, n) => {
                            if let Some(menu) = self.window_loop.menus.get(0) {
                                if self.visible.is_some() {
                                    self.pending.push_back((id, n));
                                } else {
                                    self.visible = Some(id);
                                    self.window_loop.window.send_notification(menu.id, id, n).map_err(SendNotification)?;
                                }
                            }
                        }
                        InputEvent::Shutdown => {
                            self.window_loop.join()?;
                            return Ok(Event::Shutdown);
                        }
                    }
                }
                e = self.window_loop.tick() => {
                    match e {
                        WindowEvent::MenuItemClicked(idx) => {
                            return Ok(Event::MenuItemClicked(Token::new(idx)));
                        },
                        WindowEvent::Clipboard(clipboard_event) => {
                            return Ok(Event::Clipboard(clipboard_event));
                        }
                        WindowEvent::NotificationClicked => {
                            let current = self.take_notification()?;
                            return Ok(Event::NotificationClicked(Token::new(current)));
                        }
                        WindowEvent::NotificationDismissed => {
                            let current = self.take_notification()?;
                            return Ok(Event::NotificationDismissed(Token::new(current)));
                        }
                        WindowEvent::CopyData(ty, bytes) => {
                            return Ok(Event::CopyData(ty, bytes));
                        }
                        WindowEvent::Error(error) => {
                            return Ok(Event::Error(error));
                        }
                        WindowEvent::Shutdown => {
                            self.window_loop.join()?;
                            return Ok(Event::Shutdown);
                        }
                    }
                }
            }
        }
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        _ = self.window_loop.join();
    }
}
