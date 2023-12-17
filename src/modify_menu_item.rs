/// Parameters to modify a menu item.
#[derive(Default, Debug)]
pub struct ModifyMenuItem {
    pub(super) checked: Option<bool>,
    pub(super) highlight: Option<bool>,
}

impl ModifyMenuItem {
    /// Construct a new empty modification.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the checked state of the menu item.
    pub fn checked(self, checked: bool) -> Self {
        Self {
            checked: Some(checked),
            ..self
        }
    }

    /// Set that the menu item should be highlighted.
    pub fn highlight(self, highlight: bool) -> Self {
        Self {
            highlight: Some(highlight),
            ..self
        }
    }
}
