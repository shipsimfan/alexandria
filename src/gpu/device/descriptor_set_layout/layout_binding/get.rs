use crate::gpu::{VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanShaderStageFlags};

impl VulkanDescriptorSetLayoutBinding {
    /// Get the binding index of this descriptor set layout binding
    pub fn binding(&self) -> u32 {
        self.inner.binding
    }

    /// Get the descriptor type of this descriptor set layout binding
    pub fn descriptor_type(&self) -> VulkanDescriptorType {
        self.inner.descriptor_type
    }

    /// Get the number of descriptors in this descriptor set layout binding
    pub fn descriptor_count(&self) -> u32 {
        self.inner.descriptor_count
    }

    /// Get the stage flags of this descriptor set layout binding
    pub fn stage_flags(&self) -> VulkanShaderStageFlags {
        self.inner.stage_flags
    }
}
