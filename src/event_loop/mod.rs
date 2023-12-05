use self::sender::InputEvent;
pub use self::sender::Sender;
mod sender;

pub use self::event_loop::EventLoop;
mod event_loop;

use self::window_builder::Icon;
pub use self::window_builder::WindowBuilder;
mod window_builder;

/// A token provided to callbacks to indicate what has been interacted with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token(u32);

impl Token {
    fn new(id: u32) -> Self {
        Self(id)
    }
}

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
