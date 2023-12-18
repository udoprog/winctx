//! Type used to interact with an icons collection.

use crate::{Icon, IconBuffer};

/// A collection of notification icons.
///
/// This defines the various icons that an application using winctx can use.
///
/// This is returned by [`WindowBuilder::icons`].
///
/// [`WindowBuilder::icons`]: crate::WindowBuilder::icons
#[derive(Default)]
pub struct Icons {
    pub(super) icons: Vec<IconBuffer>,
}

impl Icons {
    /// Construct a new empty collection of notification icons.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an icon from a buffer and return a handle to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use winctx::WindowBuilder;
    ///
    /// # macro_rules! include_bytes { ($path:literal) => { &[] } }
    /// const ICON: &[u8] = include_bytes!("tokio.ico");
    ///
    /// let mut window = WindowBuilder::new("se.tedro.Example");
    /// let icon = window.icons().insert_buffer(ICON, 22, 22);
    /// ```
    pub fn insert_buffer<T>(&mut self, buffer: T, width: u32, height: u32) -> Icon
    where
        T: AsRef<[u8]>,
    {
        let icon = Icon::new(self.icons.len() as u32);
        self.icons
            .push(IconBuffer::from_buffer(buffer, width, height));
        icon
    }
}
