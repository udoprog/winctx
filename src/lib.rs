//! [<img alt="github" src="https://img.shields.io/badge/github-udoprog/winctx-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/winctx)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/winctx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/winctx)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-winctx-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/winctx)
//! Asynchronous helper to install an application into a Windows context.
//!
//! This provides an asynchronous API for building a desktop application running
//! in the background which has a context menu.
//!
//! Note that crate is fairly opinionated.
//!
//! <br>
//!
//! ## Example
//!
//! ```no_run
//! use std::pin::pin;
//!
//! use tokio::signal::ctrl_c;
//! use tokio_winctx::{Event, Notification, WindowBuilder};
//!
//! const ICON: &[u8] = include_bytes!("tokio.ico");
//!
//! #[tokio::main]
//! async fn main() -> tokio_winctx::Result<()> {
//!     let mut builder = WindowBuilder::new("Example Application");
//!     builder.set_icon(ICON, 22, 22);
//!
//!     builder.add_menu_entry("Hello World", true);
//!     let notification = builder.add_menu_entry("Show notification", false);
//!     builder.add_menu_separator();
//!     let quit = builder.add_menu_entry("Quit", false);
//!
//!     let (sender, mut event_loop) = builder.build().await?;
//!
//!     let mut ctrl_c = pin!(ctrl_c());
//!     let mut shutdown = false;
//!
//!     loop {
//!         let event = tokio::select! {
//!             _ = ctrl_c.as_mut(), if !shutdown => {
//!                 sender.shutdown();
//!                 shutdown = true;
//!                 continue;
//!             }
//!             event = event_loop.tick() => {
//!                 event?
//!             }
//!         };
//!
//!         match event {
//!             Event::MenuEntryClicked(token) => {
//!                 println!("Clicked: {:?}", token);
//!
//!                 if token == notification {
//!                     sender.notification(
//!                         Notification::new("And this is a body").with_title("This is a title"),
//!                     );
//!                 }
//!
//!                 if token == quit {
//!                     sender.shutdown();
//!                 }
//!             }
//!             Event::Shutdown => {
//!                 println!("Window shut down");
//!                 break;
//!             }
//!             Event::NotificationClicked(token) => {
//!                 println!("Balloon clicked: {:?}", token);
//!             }
//!             Event::NotificationTimeout(token) => {
//!                 println!("Notification timed out: {:?}", token);
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod convert;
mod registry;
mod window;

pub use self::notification::Notification;
mod notification;

pub use self::error::Error;
mod error;

pub use self::event_loop::{Event, EventLoop, Sender, WindowBuilder};
mod event_loop;

pub use self::autostart::AutoStart;
mod autostart;

pub mod tools;

/// Result alias for winctx.
pub type Result<T, E = Error> = core::result::Result<T, E>;
