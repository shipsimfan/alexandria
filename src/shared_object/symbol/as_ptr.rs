use crate::Symbol;

impl<T> Symbol<T> {
    /// Get the underlying pointer
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}
