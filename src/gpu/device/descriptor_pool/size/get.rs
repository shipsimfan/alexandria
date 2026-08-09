use crate::gpu::{VulkanDescriptorPoolSize, VulkanDescriptorType};

impl VulkanDescriptorPoolSize {
    /// Get the type of the descriptor pool size
    pub fn r#type(&self) -> VulkanDescriptorType {
        self.inner.r#type
    }

    /// Get the count of the descriptor pool size
    pub fn count(&self) -> u32 {
        self.inner.descriptor_count
    }
}
