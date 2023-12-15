use std::ffi::{OsStr, OsString};
use std::io;
use std::mem::MaybeUninit;
use std::ptr;

use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use windows_sys::Win32::System::Registry::{self as winreg, HKEY};

use crate::convert::{FromWide, ToWide};

/// An open registry key.
///
/// This is constructed using [`OpenRegistryKey`].
pub struct RegistryKey(winreg::HKEY);

unsafe impl Sync for RegistryKey {}
unsafe impl Send for RegistryKey {}

/// Helper to open a registry key with the ability to specify desired
/// permissions.
pub struct OpenRegistryKey {
    key: HKEY,
    desired: u32,
}

impl OpenRegistryKey {
    /// Open the given key in the `HKEY_CURRENT_USER` registry.
    pub fn current_user() -> Self {
        Self {
            key: winreg::HKEY_CURRENT_USER,
            desired: winreg::KEY_READ | winreg::KEY_WOW64_64KEY,
        }
    }

    /// Open the given key in the `HKEY_LOCAL_MACHINE` registry.
    pub fn local_machine() -> Self {
        Self {
            key: winreg::HKEY_LOCAL_MACHINE,
            desired: winreg::KEY_READ,
        }
    }

    /// Enable the `KEY_SET_VALUE` desired access mode.
    pub fn set_value(mut self) -> Self {
        self.desired |= winreg::KEY_SET_VALUE;
        self
    }

    /// Internal open implementation.
    pub fn open<K>(self, key: K) -> io::Result<RegistryKey>
    where
        K: AsRef<OsStr>,
    {
        let key = key.to_wide_null();
        self.open_inner(&key)
    }

    fn open_inner(&self, key: &[u16]) -> io::Result<RegistryKey> {
        unsafe {
            let mut hkey = MaybeUninit::uninit();

            let status =
                winreg::RegOpenKeyExW(self.key, key.as_ptr(), 0, self.desired, hkey.as_mut_ptr());

            if status != ERROR_SUCCESS {
                return Err(io::Error::from_raw_os_error(status as i32));
            }

            Ok(RegistryKey(hkey.assume_init()))
        }
    }
}

impl RegistryKey {
    /// Open the given key in the HKEY_CURRENT_USER scope.
    #[inline]
    pub fn current_user<K>(key: K) -> io::Result<RegistryKey>
    where
        K: AsRef<OsStr>,
    {
        OpenRegistryKey::current_user().open(key)
    }

    /// Open the given key in the HKEY_LOCAL_MACHINE scope.
    pub fn local_machine<K>(key: K) -> io::Result<RegistryKey>
    where
        K: AsRef<OsStr>,
    {
        OpenRegistryKey::local_machine().open(key)
    }

    /// Get the given value as a string.
    pub fn get_string<N>(&self, name: N) -> io::Result<OsString>
    where
        N: AsRef<OsStr>,
    {
        let name = name.to_wide_null();
        let bytes = self.get_wide(&name, winreg::RRF_RT_REG_SZ)?;
        // Skip the terminating null.
        Ok(OsString::from_wide(&bytes[..bytes.len().saturating_sub(1)]))
    }

    fn get_wide(&self, name: &[u16], flags: u32) -> io::Result<Vec<u16>> {
        let mut len = MaybeUninit::uninit();
        let mut len2 = MaybeUninit::uninit();

        unsafe {
            let status = winreg::RegGetValueW(
                self.0,
                ptr::null_mut(),
                name.as_ptr(),
                flags,
                ptr::null_mut(),
                ptr::null_mut(),
                len.as_mut_ptr(),
            );

            if status != ERROR_SUCCESS {
                return Err(io::Error::from_raw_os_error(status as i32));
            }

            let len = len.assume_init();

            let mut value = vec![0u16; (len / 2) as usize];

            let status = winreg::RegGetValueW(
                self.0,
                ptr::null_mut(),
                name.as_ptr(),
                flags,
                ptr::null_mut(),
                value.as_mut_ptr().cast(),
                len2.as_mut_ptr(),
            );

            if status != ERROR_SUCCESS {
                return Err(io::Error::from_raw_os_error(status as i32));
            }

            let len2 = len2.assume_init();

            debug_assert!(len2 % 2 == 0);

            // Length reported *including* wide null terminator.
            value.truncate((len2 / 2) as usize);
            Ok(value)
        }
    }

    /// Set the given value.
    pub fn set<N>(&self, name: N, value: impl AsRef<OsStr>) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let name = name.to_wide_null();
        let value = value.to_wide_null();
        self.set_inner(&name, &value)
    }

    fn set_inner(&self, name: &[u16], value: &[u16]) -> io::Result<()> {
        let value_len = value
            .len()
            .checked_mul(2)
            .and_then(|n| u32::try_from(n).ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Value size overflow"))?;

        let status = unsafe {
            winreg::RegSetValueExW(
                self.0,
                name.as_ptr(),
                0,
                winreg::REG_SZ,
                value.as_ptr().cast(),
                value_len,
            )
        };

        if status != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Delete the given registry key.
    pub fn delete<N>(&self, name: N) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let name = name.to_wide_null();
        self.delete_inner(&name)
    }

    fn delete_inner(&self, name: &[u16]) -> io::Result<()> {
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
