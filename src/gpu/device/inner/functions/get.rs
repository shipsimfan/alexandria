use crate::gpu::device::{VulkanDeviceFunctions, VulkanSwapchainFunctions};

impl VulkanDeviceFunctions {
    /// Get the functions for swapchains
    pub fn swapchain(&self) -> &VulkanSwapchainFunctions {
        self.swapchain.as_ref().unwrap()
    }
}
