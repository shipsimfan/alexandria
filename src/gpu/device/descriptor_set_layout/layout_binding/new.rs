use crate::gpu::{VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanShaderStageFlags};
use std::ptr::null;
use vulkan::VkDescriptorSetLayoutBinding;

impl VulkanDescriptorSetLayoutBinding {
    /// Create a new [`VulkanDescriptorSetLayoutBinding`]
    pub fn new<F: Into<VulkanShaderStageFlags>>(
        binding: u32,
        descriptor_type: VulkanDescriptorType,
        descriptor_count: u32,
        stage_flags: F,
    ) -> VulkanDescriptorSetLayoutBinding {
        VulkanDescriptorSetLayoutBinding {
            inner: VkDescriptorSetLayoutBinding {
                binding,
                descriptor_type,
                descriptor_count,
                stage_flags: stage_flags.into(),
                immutable_samplers: null(),
            },
        }
    }
}
