/// An identifier for a menu item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MenuItemId(u32);

impl MenuItemId {
    #[inline]
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub(crate) const fn id(&self) -> u32 {
        self.0
    }
}
