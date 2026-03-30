use crate::gpu::device::{SwapchainFunctions, VulkanDeviceFunctions};

impl VulkanDeviceFunctions {
    /// Get the functions for swapchains
    pub fn swapchain(&self) -> &SwapchainFunctions {
        self.swapchain.as_ref().unwrap()
    }
}
