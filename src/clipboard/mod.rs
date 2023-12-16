pub(super) use self::clipboard_format::ClipboardFormat;
mod clipboard_format;

use std::ffi::c_void;
use std::io;
use std::marker::PhantomData;
use std::ops::Range;
use std::slice;

use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::Foundation::{FALSE, HANDLE, HWND};
use windows_sys::Win32::System::DataExchange::GetUpdatedClipboardFormats;
use windows_sys::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
use windows_sys::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

/// An open clipboard handle.
pub(crate) struct Clipboard;

impl Clipboard {
    /// Construct a new clipboard around the given window.
    ///
    /// # Safety
    ///
    /// The window handle must be valid and no other component must've acquired the clipboard.
    pub(super) unsafe fn new(handle: HWND) -> io::Result<Self> {
        if OpenClipboard(handle) == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(Self)
    }

    /// Enumerate available clipboard formats.
    pub(super) fn updated_formats<const N: usize>() -> UpdatedFormats<N> {
        unsafe {
            let mut formats = [0u32; N];
            let mut actual = 0;
            GetUpdatedClipboardFormats(formats.as_mut_ptr(), 16, &mut actual);

            UpdatedFormats {
                formats,
                range: 0..actual as usize,
            }
        }
    }

    /// Acquire data with the specified format.
    pub(crate) fn data(&self, format: ClipboardFormat) -> io::Result<Data<'_>> {
        // SAFETY: This is safe as long as construction is correct.
        unsafe {
            let handle = GetClipboardData(format.as_u16() as u32);

            if handle == 0 || handle == INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            }

            Ok(Data {
                handle,
                _marker: PhantomData,
            })
        }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
        }
    }
}

/// A clipboard data handle.
pub(super) struct Data<'a> {
    handle: HANDLE,
    _marker: PhantomData<&'a Clipboard>,
}

impl<'c> Data<'c> {
    pub(super) fn lock(&self) -> io::Result<Lock<'_>> {
        // SAFETY: Construction of Clipboard ensures that this is used
        // correctly.
        unsafe {
            let handle = GlobalLock(self.handle as *mut _);

            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }

            Ok(Lock {
                handle,
                _marker: PhantomData,
            })
        }
    }
}

pub(super) struct Lock<'a> {
    handle: *mut c_void,
    _marker: PhantomData<&'a ()>,
}

impl Lock<'_> {
    /// Coerce locked data into a byte slice.
    pub(super) fn as_slice(&self) -> &[u8] {
        // SAFETY: Lock has been correctly acquired.
        unsafe {
            let len = GlobalSize(self.handle) as usize;
            slice::from_raw_parts(self.handle.cast(), len)
        }
    }

    /// Coerce locked data into a wide slice.
    pub(super) fn as_wide_slice(&self) -> &[u16] {
        // SAFETY: Lock has been correctly acquired.
        unsafe {
            let len = GlobalSize(self.handle) as usize;
            debug_assert!(len % 2 == 0, "a wide slice must be a multiple of two");
            slice::from_raw_parts(self.handle.cast(), len / 2)
        }
    }
}

impl Drop for Lock<'_> {
    fn drop(&mut self) {
        // SAFETY: Lock has been correctly acquired.
        unsafe {
            let _ = GlobalUnlock(self.handle);
        }
    }
}

/// An iterator over clipboard formats.
pub(super) struct UpdatedFormats<const N: usize> {
    formats: [u32; N],
    range: Range<usize>,
}

impl<const N: usize> Iterator for UpdatedFormats<N> {
    type Item = ClipboardFormat;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        let format = self.formats[index];
        Some(ClipboardFormat::new(format as u16))
    }
}
