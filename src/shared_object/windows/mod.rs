use std::path::PathBuf;
use win32::HMODULE;

mod drop;
mod get;
mod load_raw_symbol;
mod open;

/// The definition of a shared object on Windows
pub(in crate::shared_object) struct SharedObjectInner {
    /// The handle to the shared object
    handle: HMODULE,

    /// The name of this shared object
    name: PathBuf,
}
