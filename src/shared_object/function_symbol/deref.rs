use crate::FunctionSymbol;
use std::ops::Deref;

impl<F> Deref for FunctionSymbol<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.r#fn
    }
}
