use crate::{AlexandriaContext, Result, context::AlexandriaContextInner};
use std::{mem::MaybeUninit, sync::Arc};

impl AlexandriaContext {
    /// Create a new [`AlexandriaContext`]
    pub(in crate::context) fn new(gpu: bool, window: bool) -> Result<AlexandriaContext> {
        let mut result = Ok(());
        let context =
            Arc::new_cyclic(
                |context| match AlexandriaContextInner::new(context, gpu, window) {
                    Ok(alexandria_context) => MaybeUninit::new(alexandria_context),
                    Err(error) => {
                        result = Err(error);
                        MaybeUninit::uninit()
                    }
                },
            );

        Ok(AlexandriaContext {
            inner: unsafe { context.assume_init() },
        })
    }
}
