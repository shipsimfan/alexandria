use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Enable additional dynamic state in the pipeline.
    pub fn extended_dynamic_state(mut self) -> Self {
        self.extended_dynamic_state = true;
        self
    }
}
