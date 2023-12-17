/// The buffer for an image.
pub(crate) struct IconBuffer {
    buffer: Box<[u8]>,
    width: u32,
    height: u32,
}

impl IconBuffer {
    /// Construct an icon from a raw buffer.
    pub(crate) fn from_buffer<T>(buffer: T, width: u32, height: u32) -> Self
    where
        T: AsRef<[u8]>,
    {
        Self {
            buffer: buffer.as_ref().into(),
            width,
            height,
        }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }
}
