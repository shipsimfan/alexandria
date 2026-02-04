use crate::Symbol;
use std::ops::Deref;

impl<T> Deref for Symbol<T> {
    type Target = *mut T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> AsRef<*mut T> for Symbol<T> {
    fn as_ref(&self) -> &*mut T {
        &self.ptr
    }
}
