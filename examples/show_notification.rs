use std::pin::pin;

use tokio::signal::ctrl_c;
use tokio_winctx::{Event, Notification, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> tokio_winctx::Result<()> {
    let mut builder = WindowBuilder::new("Example Application");
    builder.set_icon(ICON, 22, 22);

    builder.add_menu_entry("Hello World", true);
    let notification = builder.add_menu_entry("Show notification", false);
    builder.add_menu_separator();
    let quit = builder.add_menu_entry("Quit", false);

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
            Event::MenuEntryClicked(token) => {
                println!("Clicked: {:?}", token);

                if token == notification {
                    sender.notification(
                        Notification::new("And this is a body").with_title("This is a title"),
                    );
                }

                if token == quit {
                    sender.shutdown();
                }
            }
            Event::Shutdown => {
                println!("Window shut down");
                break;
            }
            Event::NotificationClicked(token) => {
                println!("Balloon clicked: {:?}", token);
            }
            Event::NotificationTimeout(token) => {
                println!("Notification timed out: {:?}", token);
            }
            _ => {}
        }
    }

    Ok(())
}
