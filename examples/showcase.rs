use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::{ContextBuilder, Event, MenuItem, Notification};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut builder = ContextBuilder::new("se.tedro.Example").window_name("Example Application");
    builder.set_icon(ICON, 22, 22);

    builder.push_menu_item(MenuItem::entry("Hello World", true));
    let notification = builder.push_menu_item(MenuItem::entry("Show notification", false));
    let notification_multiple =
        builder.push_menu_item(MenuItem::entry("Show multiple notifications", false));
    builder.push_menu_item(MenuItem::separator());
    let quit = builder.push_menu_item(MenuItem::entry("Quit", false));

    let (sender, mut event_loop) = builder.build().await?;

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
            Event::MenuItemClicked(token) => {
                println!("Menu entry clicked: {:?}", token);

                if token == notification {
                    sender.notification(
                        Notification::new("And this is a body")
                            .title("This is a title")
                            .large_icon(),
                    );
                    continue;
                }

                if token == notification_multiple {
                    sender.notification(Notification::new("First"));
                    sender.notification(Notification::new("Second"));
                    continue;
                }

                if token == quit {
                    sender.shutdown();
                }
            }
            Event::NotificationClicked(token) => {
                println!("Balloon clicked: {:?}", token);
            }
            Event::NotificationDismissed(token) => {
                println!("Notification dismissed: {:?}", token);
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
