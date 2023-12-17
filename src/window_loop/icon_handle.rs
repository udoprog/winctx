use std::io;

use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::UI::WindowsAndMessaging as winuser;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, HICON};

#[derive(Clone)]
pub(crate) struct IconHandle {
    pub(super) hicon: HICON,
}

impl IconHandle {
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

        let hicon = unsafe {
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

        if hicon == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { hicon })
    }
}

impl Drop for IconHandle {
    fn drop(&mut self) {
        // SAFETY: icon handle is owned by this struct.
        unsafe {
            DestroyIcon(self.hicon);
        }
    }
}
