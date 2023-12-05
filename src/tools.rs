//! Various miscellaneous tools for winctx.

use std::io;
use std::path::Path;
use std::ptr;

use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winuser::SW_SHOW;

use crate::convert::ToWide;

/// Open the given directory.
pub fn open_dir(path: &Path) -> io::Result<bool> {
    let path = path.to_wide_null();
    let operation = "open".to_wide_null();

    let result = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            operation.as_ptr(),
            path.as_ptr(),
            ptr::null(),
            ptr::null(),
            SW_SHOW,
        )
    };

    Ok(result as usize > 32)
}
