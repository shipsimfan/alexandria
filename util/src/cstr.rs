use std::ffi::CStr;

/// Converts a `slice` of [`u8`]s to a [`CStr`]
pub fn slice_to_cstr(slice: &[u8]) -> &CStr {
    CStr::from_bytes_until_nul(slice).expect("Failed to get a `CStr` from a slice")
}

/// Converts a `slice` of [`i8`]s to a [`CStr`]
pub fn i8_slice_to_cstr(slice: &[i8]) -> &CStr {
    slice_to_cstr(unsafe { std::mem::transmute(slice) })
}
