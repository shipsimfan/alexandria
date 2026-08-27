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

    /// Get a mutable reference to the type of the descriptor pool size
    pub fn r#type_mut(&mut self) -> &mut VulkanDescriptorType {
        &mut self.inner.r#type
    }

    /// Get a mutable reference to the count of the descriptor pool size
    pub fn count_mut(&mut self) -> &mut u32 {
        &mut self.inner.descriptor_count
    }
}
