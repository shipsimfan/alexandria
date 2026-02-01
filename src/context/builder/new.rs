use crate::{AlexandriaContext, AlexandriaContextBuilder};

impl AlexandriaContext {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn builder() -> AlexandriaContextBuilder {
        AlexandriaContextBuilder::new()
    }
}

impl AlexandriaContextBuilder {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn new() -> AlexandriaContextBuilder {
        AlexandriaContextBuilder {
            gpu: false,
            window: false,
        }
    }
}
