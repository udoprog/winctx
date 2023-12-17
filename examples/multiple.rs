use std::pin::pin;

use anyhow::Result;
use tokio::signal::ctrl_c;
use winctx::{Event, Icons, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> Result<()> {
    let mut icons = Icons::new();
    let default_icon = icons.push_buffer(ICON, 22, 22);

    let mut builder = WindowBuilder::new("se.tedro.Example")
        .icons(icons)
        .clipboard_events(true);

    let area1 = builder.new_area().icon(default_icon);

    let menu1 = area1.popup_menu();
    let first = menu1.push_entry("Menu 1").id();
    menu1.set_default(first);

    let area2 = builder.new_area().icon(default_icon);

    let menu2 = area2.popup_menu();
    let second = menu2.push_entry("Menu 2").id();
    menu2.set_default(first);

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
            Event::IconClicked { area_id, .. } => {
                println!("Icon clicked: {area_id:?}");
            }
            Event::MenuItemClicked { item_id, .. } => {
                println!("Menu entry clicked: {item_id:?}");

                if item_id == first {
                    println!("Menu 1 clicked");
                }

                if item_id == second {
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
