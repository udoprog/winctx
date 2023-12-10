use std::{
    ffi::{OsStr, OsString},
    marker::PhantomData,
};

pub(crate) type RawHandle = *mut ();

pub(crate) trait FromRawHandle {
    fn from_raw_handle(handle: RawHandle) -> Self;
}

pub(crate) struct OwnedHandle;
pub(crate) struct EncodeWide<'a>(PhantomData<&'a ()>);

impl Iterator for EncodeWide<'_> {
    type Item = u16;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("not implemented on this platform")
    }
}

impl FromRawHandle for OwnedHandle {
    #[inline]
    fn from_raw_handle(_: RawHandle) -> Self {
        unimplemented!("not implemented on this platform")
    }
}

pub(crate) trait OsStringExt {
    fn from_wide(wide: &[u16]) -> Self;
}

pub(crate) trait OsStrExt {
    fn encode_wide(&self) -> EncodeWide<'_> {
        unimplemented!("not implemented on this platform")
    }
}

impl OsStrExt for OsStr {
    #[inline]
    fn encode_wide(&self) -> EncodeWide<'_> {
        EncodeWide(PhantomData)
    }
}

impl OsStringExt for OsString {
    #[inline]
    fn from_wide(_: &[u16]) -> Self {
        unimplemented!("not implemented on this platform")
    }
}
