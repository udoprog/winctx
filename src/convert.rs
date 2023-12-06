use std::char::DecodeUtf16Error;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

use crate::Result;

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
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }

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

impl FromWide for OsString {
    fn from_wide(wide: &[u16]) -> OsString {
        OsStringExt::from_wide(wide)
    }
}

pub(super) fn encode_escaped_os_str(
    out: &mut String,
    input: &OsStr,
) -> Result<(), DecodeUtf16Error> {
    use std::char::decode_utf16;

    'escape: {
        for c in input.encode_wide() {
            match c {
                // ' '
                0x00000020 => break 'escape,
                _ => {}
            }
        }

        // No escaping needed.
        for c in decode_utf16(input.encode_wide()) {
            out.push(c?);
        }

        return Ok(());
    };

    out.push('"');

    for c in decode_utf16(input.encode_wide()) {
        out.push(c?);
    }

    out.push('"');
    Ok(())
}
