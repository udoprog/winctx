use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::icon::StockIcon;
use winctx::{Event, Icons, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut has_tooltip = true;
    let mut is_checked = true;
    let mut is_highlighted = true;

    let mut icons = Icons::new();
    let initial_icon = icons.push_buffer(ICON, 22, 22);

    let mut window = WindowBuilder::new("se.tedro.Example")
        .window_name("Example Application")
        .icons(icons);

    let area = window.new_area().icon(initial_icon);

    if has_tooltip {
        area.tooltip("Example Application");
    }

    let menu = area.popup_menu();

    let title = menu.push_entry("Hello World").id();
    menu.push_entry("Show notification");
    menu.push_entry("Show multiple notifications");

    menu.push_entry("Toggle tooltip").checked(has_tooltip);
    menu.push_entry("Toggle checked").checked(is_checked);

    menu.push_entry("Toggle highlighted")
        .checked(is_highlighted)
        .highlight(is_highlighted);

    menu.push_separator();

    let quit = menu.push_entry("Quit").id();

    menu.set_default(title);

    let area_id = area.id();

    let (sender, mut event_loop) = window.build().await?;

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
            Event::IconClicked { area_id, event, .. } => {
                println!("Icon clicked: {area_id:?}: {event:?}");
            }
            Event::MenuItemClicked { item_id, .. } => {
                println!("Menu entry clicked: {item_id:?}");

                match item_id {
                    winctx::item_id!(0, 1) => {
                        sender
                            .notification(area_id)
                            .title("This is a title")
                            .message("This is a body")
                            .large_icon()
                            .stock_icon(StockIcon::AUDIOFILES)
                            .icon_link_overlay()
                            .send();
                    }
                    winctx::item_id!(0, 2) => {
                        sender.notification(area_id).message("First").send();
                        sender.notification(area_id).message("Second").send();
                    }
                    winctx::item_id!(0, 3) => {
                        if has_tooltip {
                            sender.modify_area(area_id).tooltip("").send();
                        } else {
                            sender
                                .modify_area(area_id)
                                .tooltip("This is a tooltip!")
                                .send();
                        }

                        has_tooltip = !has_tooltip;
                        sender.modify_menu_item(item_id).checked(has_tooltip).send();
                    }
                    winctx::item_id!(0, 4) => {
                        is_checked = !is_checked;
                        sender.modify_menu_item(item_id).checked(is_checked).send();
                    }
                    winctx::item_id!(0, 5) => {
                        is_highlighted = !is_highlighted;
                        sender
                            .modify_menu_item(item_id)
                            .checked(is_highlighted)
                            .highlight(is_highlighted)
                            .send();
                    }
                    _ => {
                        println!("Unhandled: {item_id:?}");
                    }
                }

                if item_id == quit {
                    sender.shutdown();
                }
            }
            Event::NotificationClicked { area_id, id, .. } => {
                println!("Balloon clicked: {area_id:?}: {id:?}");
            }
            Event::NotificationDismissed { area_id, id, .. } => {
                println!("Notification dismissed: {area_id:?}: {id:?}");
            }
            Event::CopyData { ty, data, .. } => {
                println!("Data of type {ty} copied to process: {:?}", data);
            }
            Event::Shutdown { .. } => {
                println!("Window shut down");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
