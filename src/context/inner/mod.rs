use crate::{EventQueue, gpu::GpuSubsystem, window::WindowSubsystem};
use std::{cell::RefCell, time::Instant};

mod drop;
mod get;
mod new;

/// The main entry point for interacting with Alexandria
pub struct AlexandriaContextInner<UserEvent: Send> {
    /// The time the context was created
    start_time: Instant,

    /// The event queue for this context
    event_queue: EventQueue<UserEvent>,

    /// The subsystem for interacting and controling GPUs
    gpu: Option<GpuSubsystem>,

    /// The system for interacting with platform windowing systems
    window: Option<WindowSubsystem>,
}

thread_local! {
    /// A boolean to make sure only a single [`AlexandriaContext`] exists on a given thread
    static ALEXANDRIA_CONTEXT_ACTIVE: RefCell<bool> = RefCell::new(false);
}
