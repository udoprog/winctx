/// The identifier for a notification menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MenuId(u32);

impl MenuId {
    /// Construct a new menu id.
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the menu id.
    pub(crate) fn id(&self) -> u32 {
        self.0
    }
}
