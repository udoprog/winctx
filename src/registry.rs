use std::ffi::{OsStr, OsString};
use std::io;
use std::mem::MaybeUninit;
use std::ptr;

use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_sys::Win32::System::Registry as winreg;

use crate::convert::{FromWide, ToWide};

pub(crate) struct RegistryKey(winreg::HKEY);

unsafe impl Sync for RegistryKey {}
unsafe impl Send for RegistryKey {}

impl RegistryKey {
    /// Open the given key in the HKEY_CURRENT_USER scope.
    pub(crate) fn current_user(key: &str) -> io::Result<RegistryKey> {
        Self::open(winreg::HKEY_CURRENT_USER, key)
    }

    /// Internal open implementation.
    fn open(reg: winreg::HKEY, key: &str) -> io::Result<RegistryKey> {
        let key = key.to_wide_null();

        unsafe {
            let mut hkey = MaybeUninit::uninit();

            let status = winreg::RegOpenKeyExW(
                reg,
                key.as_ptr(),
                0,
                winreg::KEY_READ | winreg::KEY_SET_VALUE | winreg::KEY_WOW64_32KEY,
                hkey.as_mut_ptr(),
            );

            if status != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(RegistryKey(hkey.assume_init()))
        }
    }

    /// Get the given value.
    pub(crate) fn get(&self, name: &str) -> io::Result<Option<OsString>> {
        let name = name.to_wide_null();
        let mut len = MaybeUninit::uninit();
        let mut len2 = MaybeUninit::uninit();

        unsafe {
            let status = winreg::RegGetValueW(
                self.0,
                ptr::null_mut(),
                name.as_ptr(),
                winreg::RRF_RT_REG_SZ,
                ptr::null_mut(),
                ptr::null_mut(),
                len.as_mut_ptr(),
            );

            if status == ERROR_FILE_NOT_FOUND {
                return Ok(None);
            }

            if status != 0 {
                return Err(io::Error::last_os_error());
            }

            let len = len.assume_init();

            let mut value = vec![0; len as usize / 2];

            let status = unsafe {
                winreg::RegGetValueW(
                    self.0,
                    ptr::null_mut(),
                    name.as_ptr(),
                    winreg::RRF_RT_REG_SZ,
                    ptr::null_mut(),
                    value.as_mut_ptr().cast(),
                    len2.as_mut_ptr(),
                )
            };

            if status != 0 {
                return Err(io::Error::last_os_error());
            }

            let len2 = len2.assume_init();

            debug_assert_eq!(len, len2 as usize / 2);
            value.truncate(len2);
            Ok(Some(OsString::from_wide_null(&value)))
        }
    }

    /// Set the given value.
    pub(crate) fn set(&self, name: &str, value: impl AsRef<OsStr>) -> io::Result<()> {
        use std::convert::TryInto as _;

        let name = name.to_wide_null();
        let value = value.to_wide_null();
        let value_len: u32 = (value.len() * 2)
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "value too large"))?;

        let status = unsafe {
            winreg::RegSetValueExW(
                self.0,
                name.as_ptr(),
                0,
                winreg::REG_SZ,
                value.as_ptr() as *const u8,
                value_len,
            )
        };

        if status != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Delete the given value.
    pub(crate) fn delete(&self, name: &str) -> io::Result<()> {
        let name = name.to_wide_null();

        let status = unsafe { winreg::RegDeleteKeyValueW(self.0, ptr::null_mut(), name.as_ptr()) };

        if status != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        unsafe {
            winreg::RegCloseKey(self.0);
        }
    }
}
