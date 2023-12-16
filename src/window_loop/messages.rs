use windows_sys::Win32::UI::WindowsAndMessaging::WM_USER;

// Icon message.
pub(super) const ICON_ID: u32 = WM_USER + 1;
// Transfer bytes payload.
pub(super) const BYTES_ID: u32 = WM_USER + 2;
