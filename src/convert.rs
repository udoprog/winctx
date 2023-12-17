use std::char::decode_utf16;
use std::char::DecodeUtf16Error;
use std::ffi::{OsStr, OsString};

use crate::windows::{OsStrExt, OsStringExt};
use crate::Result;

/// Copy a wide string from a source to a destination.
pub(crate) fn copy_wstring(dest: &mut [u16], source: &str) {
    let source = source.to_wide_null();
    let len = usize::min(source.len(), dest.len());
    dest[..len].copy_from_slice(&source[..len]);
}

pub(crate) trait ToWide {
    /// Encode into a wide string.
    fn to_wide(&self) -> Vec<u16>;

    /// Encode into a null-terminated wide string.
    fn to_wide_null(&self) -> Vec<u16>;
}

impl<T> ToWide for T
where
    T: AsRef<OsStr>,
{
    #[inline]
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }

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

    fn from_wide_null(wide: &[u16]) -> Self {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        Self::from_wide(&wide[..len])
    }
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
