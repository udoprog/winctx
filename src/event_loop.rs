use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::item_id::ItemId;
use crate::window_loop::IconHandle;
use crate::window_loop::{WindowEvent, WindowLoop};
use crate::NotificationId;
use crate::{AreaId, Event, InputEvent, Notification, Result};

/// The event loop being run.
#[repr(C)]
pub struct EventLoop {
    events_rx: mpsc::UnboundedReceiver<InputEvent>,
    window_loop: WindowLoop,
    icons: Vec<IconHandle>,
    visible: Option<(AreaId, NotificationId)>,
    pending: VecDeque<(AreaId, NotificationId, Notification)>,
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

    fn take_notification(&mut self) -> Result<(AreaId, NotificationId)> {
        let (area_id, id) = self.visible.take().ok_or(MissingNotification)?;

        if let Some((area_id, id, n)) = self.pending.pop_front() {
            self.visible = Some((area_id, id));
            self.window_loop
                .window
                .send_notification(area_id, n)
                .map_err(SendNotification)?;
        }

        Ok((area_id, id))
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
                        InputEvent::ModifyArea { area_id, modify } => {
                            let icon = modify.icon.and_then(|icon| self.icons.get(icon.as_usize()));
                            self.window_loop.window.modify_notification(area_id, icon, modify.tooltip.as_deref()).map_err(ModifyNotification)?;
                        }
                        InputEvent::ModifyMenuItem { item_id, modify } => {
                            let Some(menu) = self.window_loop.areas.get(item_id.area_id().id() as usize) else {
                                continue;
                            };

                            let Some(popup_menu) = &menu.popup_menu else {
                                continue;
                            };

                            popup_menu.modify_menu_item(item_id.id(), &modify).map_err(ModifyMenuItem)?;
                        }
                        InputEvent::Notification { area_id, notification_id, notification } => {
                            if self.visible.is_some() {
                                self.pending.push_back((area_id, notification_id, notification));
                            } else {
                                self.visible = Some((area_id, notification_id));
                                self.window_loop.window.send_notification(area_id, notification).map_err(SendNotification)?;
                            }
                        }
                        InputEvent::Shutdown => {
                            self.window_loop.join()?;
                            return Ok(Event::Shutdown {});
                        }
                    }
                }
                e = self.window_loop.tick() => {
                    match e {
                        WindowEvent::MenuItemClicked(area_id, idx, event) => {
                            return Ok(Event::MenuItemClicked {
                                item_id: ItemId::new(area_id.id(), idx),
                                event,
                            });
                        },
                        WindowEvent::Clipboard(event) => {
                            return Ok(Event::Clipboard { event });
                        }
                        WindowEvent::IconClicked(area_id, event) => {
                            return Ok(Event::IconClicked { area_id, event });
                        }
                        WindowEvent::NotificationClicked(actual_menu_id, event) => {
                            let (area_id, id) = self.take_notification()?;
                            debug_assert_eq!(actual_menu_id, area_id);
                            return Ok(Event::NotificationClicked {
                                area_id,
                                id,
                                event,
                            });
                        }
                        WindowEvent::NotificationDismissed(actual_menu_id) => {
                            let (area_id, id) = self.take_notification()?;
                            debug_assert_eq!(actual_menu_id, area_id);
                            return Ok(Event::NotificationDismissed { area_id, id });
                        }
                        WindowEvent::CopyData(ty, data) => {
                            return Ok(Event::CopyData { ty, data });
                        }
                        WindowEvent::Error(error) => {
                            return Ok(Event::Error { error });
                        }
                        WindowEvent::Shutdown => {
                            self.window_loop.join()?;
                            return Ok(Event::Shutdown {});
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
