use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::{Event, Icons, MenuItem, Notification, NotificationMenu, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut icons = Icons::new();
    let initial_icon = icons.push_buffer(ICON, 22, 22);

    let mut menu = NotificationMenu::new().initial_icon(initial_icon);
    menu.push(MenuItem::entry("Hello World", true));
    let single = menu.push(MenuItem::entry("Show notification", false));
    let multiple = menu.push(MenuItem::entry("Show multiple notifications", false));
    let tooltip = menu.push(MenuItem::entry("Toggle tooltip", false));
    menu.push(MenuItem::separator());
    let quit = menu.push(MenuItem::entry("Quit", false));

    let mut window = WindowBuilder::new("se.tedro.Example")
        .window_name("Example Application")
        .icons(icons);

    let menu_id = window.push_notification_menu(menu);

    let (sender, mut event_loop) = window.build().await?;

    sender.set_tooltip(menu_id, "Hello!");
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
            Event::MenuItemClicked(menu_id, token) => {
                println!("Menu entry clicked: {menu_id:?}: {token:?}");

                if token == single {
                    sender.notification(
                        menu_id,
                        Notification::new()
                            .title("This is a title")
                            .message("This is a body")
                            .large_icon(),
                    );
                    continue;
                }

                if token == multiple {
                    sender.notification(menu_id, Notification::new().message("First"));
                    sender.notification(menu_id, Notification::new().message("Second"));
                    continue;
                }

                if token == quit {
                    sender.shutdown();
                }

                if token == tooltip {
                    if has_tooltip {
                        sender.clear_tooltip(menu_id);
                    } else {
                        sender.set_tooltip(menu_id, "This is a tooltip!");
                    }

                    has_tooltip = !has_tooltip;
                }
            }
            Event::NotificationClicked(menu_id, token) => {
                println!("Balloon clicked: {menu_id:?}: {token:?}");
            }
            Event::NotificationDismissed(menu_id, token) => {
                println!("Notification dismissed: {menu_id:?}: {token:?}");
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
