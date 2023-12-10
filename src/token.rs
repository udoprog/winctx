/// A token provided to callbacks to indicate what has been interacted with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token(u32);

impl Token {
    #[inline]
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }
}
