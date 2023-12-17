/// Parameters to modify a menu item.
#[derive(Default, Debug)]
pub(super) struct ModifyMenuItem {
    pub(super) checked: Option<bool>,
    pub(super) highlight: Option<bool>,
}

impl ModifyMenuItem {
    /// Set the checked state of the menu item.
    pub(super) fn checked(&mut self, checked: bool) {
        self.checked = Some(checked);
    }

    /// Set that the menu item should be highlighted.
    pub(super) fn highlight(&mut self, highlight: bool) {
        self.highlight = Some(highlight);
    }
}
