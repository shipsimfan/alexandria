use crate::gpu::{VulkanDescriptorPoolSize, VulkanDescriptorType};

impl VulkanDescriptorPoolSize {
    /// Set the type of the descriptor pool size
    pub fn set_type(mut self, r#type: VulkanDescriptorType) -> Self {
        self.inner.r#type = r#type;
        self
    }

    /// Set the count of the descriptor pool size
    pub fn set_count(mut self, count: u32) -> Self {
        self.inner.descriptor_count = count;
        self
    }
}
