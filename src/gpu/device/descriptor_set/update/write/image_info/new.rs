use crate::gpu::{VulkanDescriptorImageInfo, VulkanImageLayout, VulkanImageView, VulkanSampler};
use std::marker::PhantomData;
use vulkan::VkDescriptorImageInfo;

impl<'a> VulkanDescriptorImageInfo<'a> {
    /// Create a new [`VulkanDescriptorImageInfo`]
    pub fn new(
        sampler: &'a VulkanSampler,
        image_view: &'a VulkanImageView,
        image_layout: VulkanImageLayout,
    ) -> VulkanDescriptorImageInfo<'a> {
        VulkanDescriptorImageInfo {
            inner: VkDescriptorImageInfo {
                sampler: sampler.handle(),
                image_view: image_view.handle(),
                image_layout,
            },
            _marker: PhantomData,
        }
    }
}
