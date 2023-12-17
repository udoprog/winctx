use std::pin::pin;

use anyhow::Result;
use tokio::signal::ctrl_c;
use winctx::{Event, Icons, MenuItem, NotificationArea, PopupMenu, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> Result<()> {
    let mut icons = Icons::new();
    let default_icon = icons.push_buffer(ICON, 22, 22);

    let mut menu1 = PopupMenu::new();
    menu1.push(MenuItem::entry("Menu 1", true));

    let mut menu2 = PopupMenu::new();
    menu2.push(MenuItem::entry("Menu 2", true));

    let mut builder = WindowBuilder::new("se.tedro.Example")
        .icons(icons)
        .clipboard_events(true);

    let menu1 = builder.push_notification_area(
        NotificationArea::new()
            .initial_icon(default_icon)
            .popup_menu(menu1),
    );

    let menu2 = builder.push_notification_area(
        NotificationArea::new()
            .initial_icon(default_icon)
            .popup_menu(menu2),
    );

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
            Event::IconClicked(area_id) => {
                println!("Icon clicked: {area_id:?}");
            }
            Event::MenuItemClicked(area_id, token) => {
                println!("Menu entry clicked: {area_id:?}: {token:?}");

                if area_id == menu1 {
                    println!("Menu 1 clicked");
                }

                if area_id == menu2 {
                    println!("Menu 2 clicked");
                }
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
