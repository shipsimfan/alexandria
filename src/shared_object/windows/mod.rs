use std::path::PathBuf;
use win32::HMODULE;

mod drop;
mod get;
mod load_raw_symbol;
mod open;

/// The definition of a shared object on Windows
pub struct SharedObjectInner {
    /// The handle to the shared object
    handle: HMODULE,

    /// The name of this shared object
    name: PathBuf,
}
