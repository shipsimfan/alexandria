use crate::gpu::device::{VulkanDeviceFunctions, VulkanDeviceInner};
use vulkan::VkDevice;

impl VulkanDeviceInner {
    /// Get accesss to the device handle
    pub fn handle(&self) -> VkDevice {
        self.handle
    }

    /// Get the device level functions
    pub fn functions(&self) -> &VulkanDeviceFunctions {
        &self.functions
    }
}
