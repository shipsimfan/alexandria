use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;
use vulkan::VK_TRUE;

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Get whether additional dynamic state in the pipeline is enabled
    pub fn extended_dynamic_state(&self) -> bool {
        self.inner.extended_dynamic_state == VK_TRUE
    }
}
