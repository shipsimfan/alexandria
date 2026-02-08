use crate::{AlexandriaContextInner, EventQueue, gpu::GpuSubsystem, window::WindowSubsystem};
use std::time::Instant;

impl<UserEvent: Send> AlexandriaContextInner<UserEvent> {
    /// Get the time the context was created at
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    /// Get a reference to the event queue
    pub fn event_queue(&self) -> &EventQueue<UserEvent> {
        &self.event_queue
    }

    /// Get a reference to the GPU subsystem, if its been initialized
    pub fn gpu_opt(&self) -> Option<&GpuSubsystem> {
        self.gpu.as_ref()
    }

    /// Get a reference to the GPU subsystem
    ///
    /// This function will panic if the GPU subsystem wasn't initialized at creation
    pub fn gpu(&self) -> &GpuSubsystem {
        self.gpu_opt().unwrap()
    }

    /// Get a reference to the windowing subsystem, if its been initialized
    pub fn window_opt(&self) -> Option<&WindowSubsystem> {
        self.window.as_ref()
    }

    /// Get a reference to the windowing subsystem
    ///
    /// This function will panic if the windowing subsystem wasn't initialized at creation
    pub fn window(&self) -> &WindowSubsystem {
        self.window_opt().unwrap()
    }
}
