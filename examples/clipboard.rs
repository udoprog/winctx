use std::io::Cursor;
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

    builder.new_area().icon(default_icon);

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
            Event::Clipboard(clipboard_event) => match clipboard_event {
                winctx::ClipboardEvent::BitMap(bitmap) => {
                    let decoder = image::codecs::bmp::BmpDecoder::new_without_file_header(
                        Cursor::new(&bitmap[..]),
                    )?;
                    let image = image::DynamicImage::from_decoder(decoder)?;
                    image.save("clipboard.png")?;
                    println!("Saved clipboard image to clipboard.png");
                }
                winctx::ClipboardEvent::Text(text) => {
                    println!("Clipboard text: {text:?}");
                }
                _ => {}
            },
            Event::Shutdown => {
                println!("Window shut down");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
