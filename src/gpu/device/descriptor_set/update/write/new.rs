use crate::gpu::{
    VulkanDescriptorBufferInfo, VulkanDescriptorImageInfo, VulkanDescriptorSet,
    VulkanDescriptorType, VulkanWriteDescriptorSet,
};
use std::marker::PhantomData;
use vulkan::VkWriteDescriptorSet;

impl<'a> VulkanWriteDescriptorSet<'a> {
    /// Create a new [`VulkanWriteDescriptorSet`]
    pub fn new(
        dst_set: &'a VulkanDescriptorSet,
        dst_binding: u32,
        dst_array_element: u32,
        descriptor_type: VulkanDescriptorType,
        image_info: &'a [VulkanDescriptorImageInfo<'a>],
        buffer_info: &'a [VulkanDescriptorBufferInfo<'a>],
    ) -> VulkanWriteDescriptorSet<'a> {
        VulkanWriteDescriptorSet {
            inner: VkWriteDescriptorSet {
                dst_set: dst_set.handle(),
                dst_binding,
                dst_array_element,
                descriptor_count: image_info.len() as u32 + buffer_info.len() as u32,
                descriptor_type: descriptor_type.into(),
                image_info: image_info.as_ptr().cast(),
                buffer_info: buffer_info.as_ptr().cast(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
