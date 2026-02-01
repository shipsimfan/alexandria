use crate::{AlexandriaContext, AlexandriaContextBuilder};
use std::marker::PhantomData;

impl AlexandriaContext {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn builder() -> AlexandriaContextBuilder {
        AlexandriaContextBuilder::new()
    }
}

impl AlexandriaContextBuilder {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn new() -> AlexandriaContextBuilder {
        AlexandriaContextBuilder { _priv: PhantomData }
    }
}
