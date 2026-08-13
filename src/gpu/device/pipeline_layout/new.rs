use crate::{
    Error, Result,
    gpu::{
        VulkanDescriptorSetLayout, VulkanDevice, VulkanPipelineLayout,
        VulkanPipelineLayoutCreateFlags, VulkanPushConstantRange,
    },
};
use std::ptr::null;
use vulkan::{VkPipelineLayout, VkPipelineLayoutCreateInfo, try_vulkan};

impl VulkanPipelineLayout {
    /// Create a new [`VulkanPipelineLayout`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanPipelineLayoutCreateFlags,
        descriptor_set_layouts: &[&VulkanDescriptorSetLayout],
        push_constant_ranges: &[VulkanPushConstantRange],
        device: &VulkanDevice,
    ) -> Result<VulkanPipelineLayout> {
        let descriptor_set_layouts: Vec<_> = descriptor_set_layouts
            .iter()
            .map(|layout| layout.handle())
            .collect();

        let create_info = VkPipelineLayoutCreateInfo {
            flags,
            set_layout_count: descriptor_set_layouts.len() as u32,
            set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: push_constant_ranges.len() as u32,
            push_constant_ranges: push_constant_ranges.as_ptr().cast(),
            ..Default::default()
        };

        let mut handle = VkPipelineLayout::null();
        try_vulkan!((device.functions().pipeline_layout.create_pipeline_layout)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|vk| Error::new_with("unable to create a graphics pipeline", vk))?;

        Ok(VulkanPipelineLayout {
            handle,
            device: device.clone(),
        })
    }
}
