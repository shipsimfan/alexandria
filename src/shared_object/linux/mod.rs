use std::{ffi::c_void, path::PathBuf};

mod drop;
mod get;
mod load_raw_symbol;
mod open;

/// The definition of a shared object on Linux
pub(in crate::shared_object) struct SharedObjectInner {
    /// The handle to the shared object
    handle: *mut c_void,

    /// The name of this shared object
    name: PathBuf,
}
