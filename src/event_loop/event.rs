use crate::{AreaId, Error, Token};

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
    MenuItemClicked(AreaId, Token),
    /// An icon has been clicked.
    IconClicked(AreaId),
    /// Indicates that the notification with the associated token has been clicked.
    NotificationClicked(AreaId, Token),
    /// The notification associated with the given token either timed out or was dismissed.
    NotificationDismissed(AreaId, Token),
    /// The system clipboard has been modified.
    Clipboard(ClipboardEvent),
    /// Data was copied to the current process remotely using
    /// [`Window::copy_data`].
    ///
    /// [`Window::copy_data`]: crate::Window::copy_data
    CopyData(usize, Vec<u8>),
    /// Window has been shut down.
    Shutdown,
    /// A non-fatal error has been reported.
    Error(Error),
}
