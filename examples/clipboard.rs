use std::io::Cursor;
use std::pin::pin;

use anyhow::Result;
use tokio::signal::ctrl_c;
use winctx::event::ClipboardEvent;
use winctx::{Event, WindowBuilder};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> Result<()> {
    let mut window = WindowBuilder::new("se.tedro.Example").clipboard_events(true);

    let default_icon = window.icons().insert_buffer(ICON, 22, 22);

    window.new_area().icon(default_icon);

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
            Event::Clipboard { event } => match event {
                ClipboardEvent::BitMap(bitmap) => {
                    let decoder = image::codecs::bmp::BmpDecoder::new_without_file_header(
                        Cursor::new(&bitmap[..]),
                    )?;
                    let image = image::DynamicImage::from_decoder(decoder)?;
                    image.save("clipboard.png")?;
                    println!("Saved clipboard image to clipboard.png");
                }
                ClipboardEvent::Text(text) => {
                    println!("Clipboard text: {text:?}");
                }
                _ => {}
            },
            Event::Shutdown { .. } => {
                println!("Window shut down");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
