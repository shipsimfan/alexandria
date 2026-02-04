use crate::{SharedObject, Symbol};

impl<T> Symbol<T> {
    /// Unwrap the contents of this symbol
    pub unsafe fn unwrap(self) -> (*mut T, SharedObject) {
        (self.ptr, self.shared_object)
    }
}
