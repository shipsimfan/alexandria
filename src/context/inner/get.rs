use crate::{
    EventQueue, context::AlexandriaContextInner, gpu::GpuSubsystem, window::WindowSubsystem,
};
use std::time::Instant;

impl<UserEvent: 'static + Send> AlexandriaContextInner<UserEvent> {
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

    /// Get a reference to the windowing subsystem, if its been initialized
    pub fn window_opt(&self) -> Option<&WindowSubsystem<UserEvent>> {
        self.window.as_ref()
    }
}
