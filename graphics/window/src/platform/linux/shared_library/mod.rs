use std::ffi::c_void;

mod drop;
mod load;
mod try_open;

pub(in crate::platform::linux) use load::try_load_function;

/// A handle to a shared Linux library
pub(in crate::platform::linux) struct SharedLibrary {
    /// The raw handle to the library
    handle: *mut c_void,

    /// A name for debugging purposes
    name: &'static str,
}
