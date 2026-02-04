use crate::{SharedObject, Symbol};

impl<T> Symbol<T> {
    /// Create a new [`Symbol`] from `ptr`
    pub(in crate::shared_object) fn new(ptr: *mut T, shared_object: SharedObject) -> Symbol<T> {
        Symbol { ptr, shared_object }
    }
}
