use crate::SharedObject;

mod deref;
mod get;
mod new;

/// A symbol for a function
#[derive(Clone)]
pub struct FunctionSymbol<F> {
    /// The function being pointed at
    r#fn: F,

    /// The shared object this symbol came from
    shared_object: SharedObject,
}
