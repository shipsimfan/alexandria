use crate::gpu::VulkanDeviceVulkan13Features;

impl Default for VulkanDeviceVulkan13Features {
    fn default() -> Self {
        VulkanDeviceVulkan13Features {
            synchorization2: false,
            dynamic_rendering: false,
        }
    }
}
