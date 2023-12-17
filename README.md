# winctx

[<img alt="github" src="https://img.shields.io/badge/github-udoprog/winctx-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/winctx)
[<img alt="crates.io" src="https://img.shields.io/crates/v/winctx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/winctx)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-winctx-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/winctx)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/udoprog/winctx/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/udoprog/winctx/actions?query=branch%3Amain)

A minimal window context for Rust.

In order to do most productive things in a Windows desktop environment, you
need to construct and interact with a window. Constructing this window
allows for processing messages which fill a wide range of functions.

Doing this allows applications to:
* [Register and use a context menu, the icons you see in the bottom right
  for running applications][showcase].
* [Send notifcations, or balloons as Windows call them][showcase].
* [Monitor the clipboard for changes][clipboard].
* [Copy data to a remote process][copy-data], allowing for very simple
  unidirection IPC.

There are a few additional APIs provided by this crate because they are so
common:
* [Basic access the registry][registry] allowing the registration of an
  application that should be [started automatically].

Note that crate is fairly opinionated, not everything that is possible
through the underlying APIs will be exposed.

<br>

## Example

The primary purpose of this crate is to:
* Define a window and its capabilities. I.e. if it should have a context
  menu or receive clipboard events.
* Handle incoming [Events][Event] from the window.

The basic loop looks like this:

```rust
use std::pin::pin;

use tokio::signal::ctrl_c;
use winctx::{Event, Notification, WindowBuilder, MenuItem};

const ICON: &[u8] = include_bytes!("tokio.ico");

#[tokio::main]
async fn main() -> winctx::Result<()> {
    let mut builder = WindowBuilder::new("se.tedro.Example");
    builder.set_icon(ICON, 22, 22);

    builder.push(MenuItem::entry("Hello World", true));
    let notification = builder.push(MenuItem::entry("Show notification", false));
    let notification_multiple = builder.push(MenuItem::entry("Show multiple notifications", false));
    builder.push(MenuItem::separator());
    let quit = builder.push(MenuItem::entry("Quit", false));

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
            Event::MenuItemClicked(area_id, token) => {
                println!("Menu entry clicked: {area_id:?}: {token:?}");

                if token == notification {
                    sender.notification(
                        Notification::new("This is a body").title("This is a title"),
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
            Event::Shutdown => {
                println!("Window shut down");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
```

[Event]: https://docs.rs/winctx/latest/winctx/enum.Event.html
[clipboard]: https://github.com/udoprog/winctx/blob/main/examples/clipboard.rs
[copy-data]: https://github.com/udoprog/winctx/blob/main/examples/copy_data.rs
[registry]: https://github.com/udoprog/winctx/blob/main/examples/registry.rs
[showcase]: https://github.com/udoprog/winctx/blob/main/examples/showcase.rs
[started automatically]: https://docs.rs/winctx/latest/winctx/struct.AutoStart.html
