//! Types related to icons.

#[doc(inline)]
pub use self::stock_icon::StockIcon;
mod stock_icon;

/// A reference to an icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon(u32);

impl Icon {
    #[inline]
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub(crate) fn as_usize(self) -> usize {
        self.0 as usize
    }
}
