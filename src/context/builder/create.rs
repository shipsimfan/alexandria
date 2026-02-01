use crate::{AlexandriaContext, AlexandriaContextBuilder, Result};

impl AlexandriaContextBuilder {
    /// Create a new [`AlexandriaContext`] with the provided options
    pub fn create(self) -> Result<AlexandriaContext> {
        AlexandriaContext::new(self.gpu, self.window)
    }
}
