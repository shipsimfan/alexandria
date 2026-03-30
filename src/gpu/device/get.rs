use crate::gpu::{VulkanDevice, device::VulkanDeviceFunctions};
use vulkan::VkDevice;

impl VulkanDevice {
    /// Get accesss to the device handle
    pub(in crate::gpu::device) fn handle(&self) -> VkDevice {
        self.inner.handle()
    }

    /// Get the device level functions
    pub(in crate::gpu::device) fn functions(&self) -> &VulkanDeviceFunctions {
        self.inner.functions()
    }
}
