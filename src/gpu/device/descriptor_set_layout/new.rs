use crate::{
    Error, Result,
    gpu::{
        VulkanDescriptorSetLayout, VulkanDescriptorSetLayoutBinding,
        VulkanDescriptorSetLayoutCreateFlags, VulkanDevice,
    },
};
use std::ptr::null;
use vulkan::{VkDescriptorSetLayout, VkDescriptorSetLayoutCreateInfo, try_vulkan};

impl VulkanDescriptorSetLayout {
    /// Create a new [`VulkanDescriptorSetLayout`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanDescriptorSetLayoutCreateFlags,
        bindings: &[VulkanDescriptorSetLayoutBinding],
        device: &VulkanDevice,
    ) -> Result<VulkanDescriptorSetLayout> {
        let create_info = VkDescriptorSetLayoutCreateInfo {
            flags,
            binding_count: bindings.len() as u32,
            bindings: bindings.as_ptr().cast(),
            ..Default::default()
        };

        let mut handle = VkDescriptorSetLayout::null();
        try_vulkan!((device
            .functions()
            .descriptor_set_layout
            .create_descriptor_set_layout)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to create descriptor set layout", error))?;

        Ok(VulkanDescriptorSetLayout {
            handle,
            device: device.clone(),
        })
    }
}
