use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::{
    Area, Event, Icons, MenuItem, ModifyArea, ModifyMenuItem, Notification, PopupMenu,
    WindowBuilder,
};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut icons = Icons::new();
    let initial_icon = icons.push_buffer(ICON, 22, 22);

    let mut menu = PopupMenu::new();
    let title = menu.push(MenuItem::entry("Hello World"));
    let single = menu.push(MenuItem::entry("Show notification"));
    let multiple = menu.push(MenuItem::entry("Show multiple notifications"));

    let tooltip =
        menu.push(MenuItem::entry("Toggle tooltip").initial(ModifyMenuItem::new().checked(true)));
    let checked =
        menu.push(MenuItem::entry("Toggle checked").initial(ModifyMenuItem::new().checked(true)));

    let highlighted = menu.push(
        MenuItem::entry("Toggle highlighted")
            .initial(ModifyMenuItem::new().checked(true).highlight(true)),
    );

    menu.push(MenuItem::separator());

    let quit = menu.push(MenuItem::entry("Quit"));

    menu.set_default(title);

    let mut window = WindowBuilder::new("se.tedro.Example")
        .window_name("Example Application")
        .icons(icons);

    window.push_area(
        Area::new()
            .initial(
                ModifyArea::new()
                    .icon(initial_icon)
                    .tooltip("Example Application"),
            )
            .popup_menu(menu),
    );

    let (sender, mut event_loop) = window.build().await?;

    let mut has_tooltip = true;
    let mut is_checked = true;
    let mut is_highlighted = true;

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
                        sender.modify_area(area_id, ModifyArea::new().tooltip(""));
                    } else {
                        sender
                            .modify_area(area_id, ModifyArea::new().tooltip("This is a tooltip!"));
                    }

                    has_tooltip = !has_tooltip;

                    sender.modify_menu_item(
                        area_id,
                        token,
                        ModifyMenuItem::new().checked(has_tooltip),
                    );
                }

                if token == checked {
                    is_checked = !is_checked;
                    sender.modify_menu_item(
                        area_id,
                        token,
                        ModifyMenuItem::new().checked(is_checked),
                    );
                }

                if token == highlighted {
                    is_highlighted = !is_highlighted;
                    sender.modify_menu_item(
                        area_id,
                        token,
                        ModifyMenuItem::new()
                            .checked(is_highlighted)
                            .highlight(is_highlighted),
                    );
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
