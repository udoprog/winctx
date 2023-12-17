use crate::{Icon, IconBuffer};

/// A collection of notification icons.
///
/// This defines the various icons that an application using winctx can use.
#[derive(Default)]
pub struct NotificationIcons {
    pub(super) icons: Vec<IconBuffer>,
}

impl NotificationIcons {
    /// Construct a new empty collection of notification icons.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an icon from a buffer and return a handle to it.
    pub fn push_buffer<T>(&mut self, buffer: T, width: u32, height: u32) -> Icon
    where
        T: AsRef<[u8]>,
    {
        let icon = Icon::new(self.icons.len() as u32);
        self.icons
            .push(IconBuffer::from_buffer(buffer, width, height));
        icon
    }
}
