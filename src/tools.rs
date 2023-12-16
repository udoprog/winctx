//! Minor tools made available for convenience.

use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::ptr;

use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

use crate::convert::ToWide;

/// Open the given directory using the default file manager, which on windows
/// would most likely be Explorer.
///
/// # Examples
///
/// ```
/// use winctx::tools;
///
/// tools::open_dir("D:\\Files")?;
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn open_dir<P>(path: P) -> io::Result<bool>
where
    P: AsRef<OsStr>,
{
    let path = path.to_wide_null();
    let operation = "open".to_wide_null();

    let result = unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            path.as_ptr(),
            ptr::null(),
            ptr::null(),
            SW_SHOW,
        )
    };

    Ok(result as usize > 32)
}
