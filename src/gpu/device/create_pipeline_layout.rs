use crate::{
    Result,
    gpu::{
        VulkanDescriptorSetLayout, VulkanDevice, VulkanPipelineLayout,
        VulkanPipelineLayoutCreateFlags, VulkanPushConstantRange,
    },
};

impl VulkanDevice {
    /// Create a new [`VulkanPipelineLayout`]
    pub fn create_pipeline_layout<F: Into<VulkanPipelineLayoutCreateFlags>>(
        &self,
        flags: F,
        descriptor_set_layout: Option<&VulkanDescriptorSetLayout>,
        push_constant_ranges: &[VulkanPushConstantRange],
    ) -> Result<VulkanPipelineLayout> {
        VulkanPipelineLayout::new(
            flags.into(),
            descriptor_set_layout,
            push_constant_ranges,
            self,
        )
    }
}
