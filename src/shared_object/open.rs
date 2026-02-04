use crate::{Result, SharedObject, shared_object::SharedObjectInner};
use std::path::Path;

impl SharedObject {
    /// Open the [`SharedObject`] at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SharedObject> {
        SharedObjectInner::open(path).map(SharedObject::from_inner)
    }
}
