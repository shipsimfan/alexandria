use crate::gpu::{VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanShaderStageFlags};

impl VulkanDescriptorSetLayoutBinding {
    /// Set the binding index of this descriptor set layout binding
    pub fn set_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    /// Set the descriptor type of this descriptor set layout binding
    pub fn set_descriptor_type(mut self, descriptor_type: VulkanDescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }

    /// Set the number of descriptors in this descriptor set layout binding
    pub fn set_descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }

    /// Set the stage flags of this descriptor set layout binding
    pub fn set_stage_flags<F: Into<VulkanShaderStageFlags>>(mut self, stage_flags: F) -> Self {
        self.inner.stage_flags = stage_flags.into();
        self
    }
}
