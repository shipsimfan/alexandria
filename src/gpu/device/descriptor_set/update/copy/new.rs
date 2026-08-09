use crate::gpu::{VulkanCopyDescriptorSet, VulkanDescriptorSet};
use std::marker::PhantomData;
use vulkan::VkCopyDescriptorSet;

impl<'a> VulkanCopyDescriptorSet<'a> {
    /// Create a new [`VulkanCopyDescriptorSet`]
    pub fn new(
        src_set: &'a VulkanDescriptorSet,
        src_binding: u32,
        src_array_element: u32,
        dst_set: &'a VulkanDescriptorSet,
        dst_binding: u32,
        dst_array_element: u32,
        descriptor_count: u32,
    ) -> VulkanCopyDescriptorSet<'a> {
        VulkanCopyDescriptorSet {
            inner: VkCopyDescriptorSet {
                src_set: src_set.handle,
                src_binding,
                src_array_element,
                dst_set: dst_set.handle,
                dst_binding,
                dst_array_element,
                descriptor_count,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
