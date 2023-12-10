use std::io;
use std::sync::Arc;

use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, HICON};

#[derive(Clone)]
pub(crate) struct Icon {
    inner: Arc<IconHandle>,
}

impl Icon {
    pub(crate) fn from_buffer(buffer: &[u8], width: u32, height: u32) -> io::Result<Self> {
        let offset = unsafe {
            winuser::LookupIconIdFromDirectoryEx(
                buffer.as_ptr(),
                TRUE,
                width as i32,
                height as i32,
                winuser::LR_DEFAULTCOLOR,
            )
        };

        if offset == 0 {
            return Err(io::Error::last_os_error());
        }

        let icon_data = &buffer[offset as usize..];

        let handle = unsafe {
            winuser::CreateIconFromResourceEx(
                icon_data.as_ptr(),
                icon_data.len() as u32,
                TRUE,
                0x30000,
                width as i32,
                height as i32,
                winuser::LR_DEFAULTCOLOR,
            )
        };

        if handle == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            inner: Arc::new(IconHandle { handle }),
        })
    }

    pub(super) fn as_raw_handle(&self) -> HICON {
        self.inner.handle
    }
}

struct IconHandle {
    handle: HICON,
}

impl Drop for IconHandle {
    fn drop(&mut self) {
        // SAFETY: icon handle is owned by this struct.
        unsafe {
            DestroyIcon(self.handle);
        }
    }
}
