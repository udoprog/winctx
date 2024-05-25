use std::char::decode_utf16;
use std::char::DecodeUtf16Error;
use std::ffi::{OsStr, OsString};

use crate::windows::{OsStrExt, OsStringExt};
use crate::Result;

/// Copy a wide string from a source to a destination, truncating if necessary.
pub(crate) fn copy_wstring_lossy(dest: &mut [u16], source: &str) {
    let mut n = 0;

    for c in source.encode_utf16().take(dest.len()) {
        dest[n] = c;
        n += 1;
    }

    if dest.len() > n {
        dest[n] = 0;
    } else {
        dest[n - 1] = 0;
    }
}

pub(crate) trait ToWide {
    /// Encode into a null-terminated wide string.
    fn to_wide_null(&self) -> Vec<u16>;
}

impl<T> ToWide for T
where
    T: AsRef<OsStr>,
{
    #[inline]
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

pub(crate) trait FromWide
where
    Self: Sized,
{
    fn from_wide(wide: &[u16]) -> Self;
}

impl FromWide for std::ffi::OsString {
    fn from_wide(wide: &[u16]) -> OsString {
        OsStringExt::from_wide(wide)
    }
}

pub(super) fn encode_escaped_os_str(
    out: &mut String,
    input: &OsStr,
) -> Result<(), DecodeUtf16Error> {
    let mut escape = false;

    for c in input.encode_wide() {
        // ' '
        if c == 0x00000020 {
            escape = true;
            break;
        }
    }

    if escape {
        out.push('"');

        for c in decode_utf16(input.encode_wide()) {
            out.push(c?);
        }

        out.push('"');
    } else {
        // No escaping needed.
        for c in decode_utf16(input.encode_wide()) {
            out.push(c?);
        }
    }

    Ok(())
}
