/// The identifier for an [`Area`].
///
/// [`Area`]: crate::area::Area
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AreaId(u32);

impl AreaId {
    /// Construct a new menu id.
    pub(crate) const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the menu id.
    pub(crate) const fn id(&self) -> u32 {
        self.0
    }
}
