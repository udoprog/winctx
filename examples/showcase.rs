use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::{Event, Icons, MenuItem, Notification, NotificationArea, PopupMenu, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut icons = Icons::new();
    let initial_icon = icons.push_buffer(ICON, 22, 22);

    let mut menu = PopupMenu::new();
    menu.push(MenuItem::entry("Hello World", true));
    let single = menu.push(MenuItem::entry("Show notification", false));
    let multiple = menu.push(MenuItem::entry("Show multiple notifications", false));
    let tooltip = menu.push(MenuItem::entry("Toggle tooltip", false));
    menu.push(MenuItem::separator());
    let quit = menu.push(MenuItem::entry("Quit", false));

    let mut window = WindowBuilder::new("se.tedro.Example")
        .window_name("Example Application")
        .icons(icons);

    let area_id = window.push_notification_area(
        NotificationArea::new()
            .initial_icon(initial_icon)
            .popup_menu(menu),
    );

    let (sender, mut event_loop) = window.build().await?;

    sender.set_tooltip(area_id, "Hello!");
    let mut has_tooltip = true;

    let mut ctrl_c = pin!(ctrl_c());
    let mut shutdown = false;

    loop {
        let event = tokio::select! {
            _ = ctrl_c.as_mut(), if !shutdown => {
                sender.shutdown();
                shutdown = true;
                continue;
            }
            event = event_loop.tick() => {
                event?
            }
        };

        match event {
            Event::IconClicked(area_id) => {
                println!("Icon clicked: {area_id:?}");
            }
            Event::MenuItemClicked(area_id, token) => {
                println!("Menu entry clicked: {area_id:?}: {token:?}");

                if token == single {
                    sender.notification(
                        area_id,
                        Notification::new()
                            .title("This is a title")
                            .message("This is a body")
                            .large_icon(),
                    );
                    continue;
                }

                if token == multiple {
                    sender.notification(area_id, Notification::new().message("First"));
                    sender.notification(area_id, Notification::new().message("Second"));
                    continue;
                }

                if token == quit {
                    sender.shutdown();
                }

                if token == tooltip {
                    if has_tooltip {
                        sender.clear_tooltip(area_id);
                    } else {
                        sender.set_tooltip(area_id, "This is a tooltip!");
                    }

                    has_tooltip = !has_tooltip;
                }
            }
            Event::NotificationClicked(area_id, token) => {
                println!("Balloon clicked: {area_id:?}: {token:?}");
            }
            Event::NotificationDismissed(area_id, token) => {
                println!("Notification dismissed: {area_id:?}: {token:?}");
            }
            Event::CopyData(ty, bytes) => {
                println!("Data of type {ty} copied to process: {:?}", bytes);
            }
            Event::Shutdown => {
                println!("Window shut down");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
