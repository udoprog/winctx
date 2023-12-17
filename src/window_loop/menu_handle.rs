use crate::AreaId;

use super::PopupMenuHandle;

#[repr(C)]
pub(crate) struct MenuHandle {
    pub(crate) area_id: AreaId,
    pub(crate) popup_menu: Option<PopupMenuHandle>,
}

impl MenuHandle {
    /// Construct a new menu handle.
    pub(crate) fn new(area_id: AreaId, popup_menu: Option<PopupMenuHandle>) -> Self {
        Self {
            area_id,
            popup_menu,
        }
    }
}
