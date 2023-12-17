/// An identifier for a notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationId(u32);

impl NotificationId {
    #[inline]
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
}
