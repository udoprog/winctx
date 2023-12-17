use crate::AreaId;

use super::PopupMenuHandle;

#[repr(C)]
pub(crate) struct MenuHandle {
    pub(crate) area_id: AreaId,
    pub(crate) popup_menu: Option<PopupMenuHandle>,
    pub(crate) initial_icon: Option<usize>,
}

impl MenuHandle {
    /// Construct a new menu handle.
    pub(crate) fn new(
        area_id: AreaId,
        popup_menu: Option<PopupMenuHandle>,
        initial_icon: Option<usize>,
    ) -> Self {
        Self {
            area_id,
            popup_menu,
            initial_icon,
        }
    }
}
