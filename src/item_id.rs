use std::fmt;

use crate::AreaId;

/// Helper macro to build a match pattern over an item id.
///
/// If an item is pushed to a popup menu in a given area it will always have a
/// consistent identifier, so this can be used to improve pattern matching over
/// which item was clicked.
///
/// # Examples
///
/// ```no_run
/// use std::pin::pin;
///
/// use tokio::signal::ctrl_c;
/// use winctx::{Event, Icons, WindowBuilder};
///
/// # macro_rules! include_bytes { ($path:literal) => { &[] } }
/// const ICON: &[u8] = include_bytes!("tokio.ico");
///
/// # async fn test() -> winctx::Result<()> {
/// let mut icons = Icons::new();
/// let icon = icons.push_buffer(ICON, 22, 22);
///
/// let mut window = WindowBuilder::new("se.tedro.Example")
///     .window_name("Example Application")
///     .icons(icons);
///
/// let area = window.new_area().icon(icon);
///
/// let menu = area.popup_menu();
/// let first = menu.push_entry("Example Application").id();
/// menu.push_entry("Quit");
/// menu.set_default(first);
///
/// let area2 = window.new_area().icon(icon);
/// let menu = area2.popup_menu();
/// menu.push_entry("Other area");
///
/// let (sender, mut event_loop) = window
///     .build()
///     .await?;
///
/// let mut ctrl_c = pin!(ctrl_c());
/// let mut shutdown = false;
///
/// loop {
///     let event = tokio::select! {
///         _ = ctrl_c.as_mut(), if !shutdown => {
///             sender.shutdown();
///             shutdown = true;
///             continue;
///         }
///         event = event_loop.tick() => {
///             event?
///         }
///     };
///
///     match event {
///         Event::MenuItemClicked(item_id) => {
///             match item_id {
///                 winctx::item_id!(0, 0) => {
///                     println!("first item clicked");
///                     assert_eq!(item_id, first);
///                 }
///                 winctx::item_id!(0, 1) => {
///                     sender.shutdown();
///                 }
///                 winctx::item_id!(1, 0) => {
///                     println!("Item clicked in second area");
///                 }
///                 _ => {}
///             }
///         }
///         Event::Shutdown => {
///             println!("Window shut down");
///             break;
///         }
///         _ => {}
///     }
/// }
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! item_id {
    ($area_id:pat, $id:pat) => {
        $crate::ItemId {
            area_id: $area_id,
            id: $id,
        }
    };
}

/// An identifier for a menu item.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId {
    #[doc(hidden)]
    pub area_id: u32,
    #[doc(hidden)]
    pub id: u32,
}

impl ItemId {
    #[inline]
    pub(crate) fn new(area_id: u32, id: u32) -> Self {
        Self { area_id, id }
    }

    #[inline]
    pub(crate) const fn area_id(&self) -> AreaId {
        AreaId::new(self.area_id)
    }

    #[inline]
    pub(crate) const fn id(&self) -> u32 {
        self.id
    }
}

impl fmt::Debug for ItemId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ItemId")
            .field(&self.area_id)
            .field(&self.id)
            .finish()
    }
}
