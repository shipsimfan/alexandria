use crate::{AlexandriaContext, gpu::GpuSubsystem, window::WindowSubsystem};

impl AlexandriaContext {
    /// Get a reference to the GPU subsystem
    ///
    /// This function will panic if the GPU subsystem wasn't initialized at creation
    pub fn gpu(&self) -> &GpuSubsystem {
        self.gpu.as_ref().unwrap()
    }

    /// Get a reference to the windowing subsystem
    ///
    /// This function will panic if the windowing subsystem wasn't initialized at creation
    pub fn window(&self) -> &WindowSubsystem {
        self.window.as_ref().unwrap()
    }
}
