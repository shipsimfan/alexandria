use crate::{
    AlexandriaContextInner, Error, Result, context::inner::ALEXANDRIA_CONTEXT_ACTIVE,
    gpu::GpuSubsystem, window::WindowSubsystem,
};
use std::{mem::MaybeUninit, sync::Weak};

impl AlexandriaContextInner {
    /// Create a new [`AlexandriaContextInner`]
    pub(in crate::context) fn new(
        context: &Weak<MaybeUninit<AlexandriaContextInner>>,
        mut gpu: bool,
        window: bool,
    ) -> Result<AlexandriaContextInner> {
        ALEXANDRIA_CONTEXT_ACTIVE.with_borrow_mut(|context_active| {
            if *context_active {
                return Err(Error::new(
                    "another Alexandria context is already active on this thread",
                ));
            }

            *context_active = true;
            Ok(())
        })?;

        // Normalize creation flags
        gpu |= window;

        // Create subsystems
        let gpu = if gpu {
            Some(GpuSubsystem::new(context.clone())?)
        } else {
            None
        };

        let window = if window {
            Some(WindowSubsystem::new()?)
        } else {
            None
        };

        Ok(AlexandriaContextInner { gpu, window })
    }
}
