use crate::device::{SwapchainFunctions, inner::GraphicsDeviceFunctions};

impl GraphicsDeviceFunctions {
    /// Get the functions for swapchains
    pub fn swapchain(&self) -> &SwapchainFunctions {
        self.swapchain.as_ref().unwrap()
    }
}
