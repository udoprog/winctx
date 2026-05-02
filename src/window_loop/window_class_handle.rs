use core::ptr;

use windows_sys::Win32::UI::WindowsAndMessaging::UnregisterClassW;

pub(super) struct WindowClassHandle {
    pub(super) class_name: Vec<u16>,
}

impl Drop for WindowClassHandle {
    fn drop(&mut self) {
        unsafe {
            UnregisterClassW(self.class_name.as_ptr(), ptr::null_mut());
        }
    }
}
