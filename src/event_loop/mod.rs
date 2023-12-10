use self::sender::InputEvent;
pub use self::sender::Sender;
mod sender;

pub use self::event_loop::EventLoop;
mod event_loop;

use self::window_builder::Icon;
pub use self::window_builder::WindowBuilder;
mod window_builder;

use crate::token::Token;

/// An event emitted by the event loop.
#[non_exhaustive]
pub enum Event {
    /// The menu item identified by [`Token`] has been clicked.
    MenuEntryClicked(Token),
    /// Indicates that the notification with the associated token has been clicked.
    NotificationClicked(Token),
    /// The notification associated with the given token either timed out or was dismissed.
    NotificationDismissed(Token),
    /// Window has been shut down.
    Shutdown,
}
