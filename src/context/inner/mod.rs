use crate::{gpu::GpuSubsystem, window::WindowSubsystem};
use std::cell::RefCell;

mod drop;
mod get;
mod new;

/// The main entry point for interacting with Alexandria
pub struct AlexandriaContextInner {
    /// The subsystem for interacting and controling GPUs
    gpu: Option<GpuSubsystem>,

    /// The system for interacting with platform windowing systems
    window: Option<WindowSubsystem>,
}

thread_local! {
    /// A boolean to make sure only a single [`AlexandriaContext`] exists on a given thread
    static ALEXANDRIA_CONTEXT_ACTIVE: RefCell<bool> = RefCell::new(false);
}
