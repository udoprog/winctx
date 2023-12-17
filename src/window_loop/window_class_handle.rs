use windows_sys::Win32::UI::WindowsAndMessaging as winuser;

pub(super) struct WindowClassHandle {
    pub(super) class_name: Vec<u16>,
}

impl Drop for WindowClassHandle {
    fn drop(&mut self) {
        unsafe {
            winuser::UnregisterClassW(self.class_name.as_ptr(), 0);
        }
    }
}
