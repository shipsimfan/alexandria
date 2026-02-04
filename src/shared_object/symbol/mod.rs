use crate::SharedObject;

mod as_ptr;
mod deref;
mod get;
mod new;
mod unwrap;

/// A symbol from a shared object
#[derive(Clone)]
pub struct Symbol<T> {
    /// The pointer to the symbol
    ptr: *mut T,

    /// The shared object this symbol came from
    shared_object: SharedObject,
}
