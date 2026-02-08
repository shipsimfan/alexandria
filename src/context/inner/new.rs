use crate::{
    AlexandriaContextInner, Error, EventQueue, Result, context::inner::ALEXANDRIA_CONTEXT_ACTIVE,
    gpu::GpuSubsystem, window::WindowSubsystem,
};
use std::time::Instant;

impl<UserEvent: Send> AlexandriaContextInner<UserEvent> {
    /// Create a new [`AlexandriaContextInner`]
    pub(in crate::context) fn new(
        mut gpu: bool,
        window: bool,
    ) -> Result<AlexandriaContextInner<UserEvent>> {
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
        let event_queue = EventQueue::new()?;

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

        Ok(AlexandriaContextInner {
            start_time: Instant::now(),
            event_queue,
            gpu,
            window,
        })
    }
}
