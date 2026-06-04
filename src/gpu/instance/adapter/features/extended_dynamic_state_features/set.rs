use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Enable additional dynamic state in the pipeline.
    pub fn enable_extended_dynamic_state(mut self) -> Self {
        self.inner.extended_dynamic_state = VK_TRUE;
        self
    }

    /// Disable additional dynamic state in the pipeline.
    pub fn disable_extended_dynamic_state(mut self) -> Self {
        self.inner.extended_dynamic_state = VK_FALSE;
        self
    }
}
