use crate::{AlexandriaContext, EventQueue, gpu::GpuSubsystem, window::WindowSubsystem};
use std::time::Instant;

impl<UserEvent: Send> AlexandriaContext<UserEvent> {
    /// Get the time the context was created at
    pub fn start_time(&self) -> Instant {
        self.inner.start_time()
    }

    /// Get a reference to the event queue
    pub fn event_queue(&self) -> &EventQueue<UserEvent> {
        self.inner.event_queue()
    }

    /// Get a reference to the GPU subsystem, if its been initialized
    pub fn gpu_opt(&self) -> Option<&GpuSubsystem> {
        self.inner.gpu_opt()
    }

    /// Get a reference to the GPU subsystem
    ///
    /// This function will panic if the GPU subsystem wasn't initialized at creation
    pub fn gpu(&self) -> &GpuSubsystem {
        self.gpu_opt().unwrap()
    }

    /// Get a reference to the windowing subsystem, if its been initialized
    pub fn window_opt(&self) -> Option<&WindowSubsystem> {
        self.inner.window_opt()
    }

    /// Get a reference to the windowing subsystem
    ///
    /// This function will panic if the windowing subsystem wasn't initialized at creation
    pub fn window(&self) -> &WindowSubsystem {
        self.window_opt().unwrap()
    }
}
