use crate::{AlexandriaContext, Error, Result, context::ALEXANDRIA_CONTEXT_ACTIVE};
use std::marker::PhantomData;

impl AlexandriaContext {
    /// Create a new [`AlexandriaContext`]
    pub(in crate::context) fn new() -> Result<AlexandriaContext> {
        ALEXANDRIA_CONTEXT_ACTIVE.with_borrow_mut(|context_active| {
            if *context_active {
                return Err(Error::new(
                    "another Alexandria context is already active on this thread",
                ));
            }

            *context_active = true;
            Ok(())
        })?;

        Ok(AlexandriaContext { _priv: PhantomData })
    }
}
