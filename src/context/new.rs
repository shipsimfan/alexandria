use crate::{
    AlexandriaContext, Error, Result, context::ALEXANDRIA_CONTEXT_ACTIVE, gpu::GpuSubsystem,
    window::WindowSubsystem,
};

impl AlexandriaContext {
    /// Create a new [`AlexandriaContext`]
    pub(in crate::context) fn new(mut gpu: bool, window: bool) -> Result<AlexandriaContext> {
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
            Some(GpuSubsystem::new()?)
        } else {
            None
        };

        let window = if window {
            Some(WindowSubsystem::new()?)
        } else {
            None
        };

        Ok(AlexandriaContext { gpu, window })
    }
}
