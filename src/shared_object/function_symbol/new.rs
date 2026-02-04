use crate::{FunctionSymbol, SharedObject};

impl<F> FunctionSymbol<F> {
    /// Create a new [`FunctionSymbol`] from `r#fn`
    pub unsafe fn new(r#fn: F, shared_object: SharedObject) -> FunctionSymbol<F> {
        FunctionSymbol {
            r#fn,
            shared_object,
        }
    }
}
