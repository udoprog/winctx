use core::fmt;
use std::io;
use std::mem::transmute;
use std::os::windows::io::{FromRawHandle, OwnedHandle};
use std::ptr;

use windows_core::PCWSTR;
use windows_sys::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS, TRUE};
use windows_sys::Win32::System::Threading::CreateMutexW;

use crate::convert::ToWide;
use crate::error::ErrorKind::*;
use crate::Result;

/// A named exclusive mutex that can be used to ensure that only one instance of
/// an application is running.
pub struct NamedMutex {
    _handle: OwnedHandle,
}

impl NamedMutex {
    /// Create a named mutex with the given name that is already acquired.
    ///
    /// Returns `None` if the mutex could not be acquired.
    ///
    /// # Errors
    ///
    /// Errors in case the named mutex could not be created.
    pub fn create_acquired<N>(name: N) -> Result<Option<Self>>
    where
        N: fmt::Display,
    {
        let name = name.to_string();
        let name = name.to_wide_null();
        let name = PCWSTR::from_raw(name.as_ptr());

        unsafe {
            let handle = CreateMutexW(ptr::null(), TRUE, name.as_ptr());

            if handle == 0 {
                return Err(CreateMutex(io::Error::last_os_error()).into());
            }

            let handle = OwnedHandle::from_raw_handle(transmute(handle));

            if GetLastError() == ERROR_ALREADY_EXISTS {
                return Ok(None);
            }

            Ok(Some(NamedMutex { _handle: handle }))
        }
    }
}
