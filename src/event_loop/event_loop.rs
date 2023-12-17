use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::token::Token;
use crate::window_loop::IconHandle;
use crate::window_loop::{WindowEvent, WindowLoop};
use crate::{MenuId, Notification, Result};

use super::{Event, InputEvent};

/// The event loop being run.
#[repr(C)]
pub struct EventLoop {
    events_rx: mpsc::UnboundedReceiver<InputEvent>,
    window_loop: WindowLoop,
    icons: Vec<IconHandle>,
    visible: Option<(MenuId, u32)>,
    pending: VecDeque<(MenuId, u32, Notification)>,
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

    fn take_notification(&mut self) -> Result<(MenuId, u32)> {
        let (menu_id, id) = self.visible.take().ok_or(MissingNotification)?;

        if let Some((menu_id, id, n)) = self.pending.pop_front() {
            self.visible = Some((menu_id, id));
            self.window_loop
                .window
                .send_notification(menu_id, n)
                .map_err(SendNotification)?;
        }

        Ok((menu_id, id))
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
                        InputEvent::ClearTooltip(menu_id) => {
                            self.window_loop.window.clear_tooltip(menu_id).map_err(ClearTooltip)?;
                        }
                        InputEvent::SetTooltip(menu_id, message) => {
                            self.window_loop.window.set_tooltip(menu_id, &message).map_err(SetTooltip)?;
                        }
                        InputEvent::SetIcon(menu_id, icon) => {
                            if let Some(icon) = self.icons.get(icon.as_usize()) {
                                self.window_loop.window.set_icon(menu_id, icon).map_err(SetIcon)?;
                            }
                        }
                        InputEvent::Notification(menu_id, id, n) => {
                            if self.visible.is_some() {
                                self.pending.push_back((menu_id, id, n));
                            } else {
                                self.visible = Some((menu_id, id));
                                self.window_loop.window.send_notification(menu_id, n).map_err(SendNotification)?;
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
                        WindowEvent::MenuItemClicked(menu_id, idx) => {
                            return Ok(Event::MenuItemClicked(menu_id, Token::new(idx)));
                        },
                        WindowEvent::Clipboard(clipboard_event) => {
                            return Ok(Event::Clipboard(clipboard_event));
                        }
                        WindowEvent::NotificationClicked(actual_menu_id) => {
                            let (menu_id, current) = self.take_notification()?;
                            debug_assert_eq!(actual_menu_id, menu_id);
                            return Ok(Event::NotificationClicked(menu_id, Token::new(current)));
                        }
                        WindowEvent::NotificationDismissed(actual_menu_id) => {
                            let (menu_id, current) = self.take_notification()?;
                            debug_assert_eq!(actual_menu_id, menu_id);
                            return Ok(Event::NotificationDismissed(menu_id, Token::new(current)));
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
