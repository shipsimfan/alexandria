use crate::{AlexandriaContext, Result, context::AlexandriaContextInner};

impl AlexandriaContext {
    /// Create a new [`AlexandriaContext`]
    pub(in crate::context) fn new(gpu: bool, window: bool) -> Result<AlexandriaContext> {
        AlexandriaContextInner::new(gpu, window).map(AlexandriaContext::from_inner)
    }
}
