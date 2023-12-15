use crate::token::Token;

/// A clipbaord event.
#[derive(Debug)]
#[non_exhaustive]
pub enum ClipboardEvent {
    /// A bitmap has been copied.
    BitMap(Vec<u8>),
    /// A string has been copied.
    Text(String),
}

/// An event emitted by the event loop.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    /// The menu item identified by [`Token`] has been clicked.
    MenuEntryClicked(Token),
    /// Indicates that the notification with the associated token has been clicked.
    NotificationClicked(Token),
    /// The notification associated with the given token either timed out or was dismissed.
    NotificationDismissed(Token),
    /// The system clipboard has been modified.
    Clipboard(ClipboardEvent),
    /// Window has been shut down.
    Shutdown,
}
