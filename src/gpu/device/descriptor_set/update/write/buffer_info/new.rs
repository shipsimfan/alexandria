use crate::gpu::{VulkanBuffer, VulkanDescriptorBufferInfo};
use std::marker::PhantomData;
use vulkan::VkDescriptorBufferInfo;

impl<'a> VulkanDescriptorBufferInfo<'a> {
    /// Create a new [`VulkanDescriptorBufferInfo`]
    pub fn new(
        buffer: &'a VulkanBuffer,
        offset: u64,
        range: u64,
    ) -> VulkanDescriptorBufferInfo<'a> {
        VulkanDescriptorBufferInfo {
            inner: VkDescriptorBufferInfo {
                buffer: buffer.handle(),
                offset,
                range,
            },
            _marker: PhantomData,
        }
    }
}
