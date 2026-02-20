use crate::{
    Error, EventQueue, Result,
    context::{AlexandriaContextInner, inner::ALEXANDRIA_CONTEXT_ACTIVE},
    gpu::GpuSubsystem,
    window::WindowSubsystem,
};
use std::{sync::atomic::Ordering, time::Instant};

impl<UserEvent: 'static + Send> AlexandriaContextInner<UserEvent> {
    /// Create a new [`AlexandriaContextInner`]
    pub fn new(mut gpu: bool, window: bool) -> Result<AlexandriaContextInner<UserEvent>> {
        if ALEXANDRIA_CONTEXT_ACTIVE
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err(Error::new(
                "another Alexandria context is already active on this thread",
            ));
        }

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
            Some(WindowSubsystem::new(event_queue.clone())?)
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
