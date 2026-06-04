use crate::gpu::{
    VulkanAccessFlags, VulkanImage, VulkanImageAspectFlags, VulkanImageLayout,
    VulkanImageMemoryBarrier, VulkanPipelineStageFlags,
};
use std::marker::PhantomData;
use vulkan::{VkImageMemoryBarrier2, VkImageSubresourceRange};

impl<'a> VulkanImageMemoryBarrier<'a> {
    /// Create a new [`VulkanImageMemoryBarrier`]
    pub fn new<
        P1: Into<VulkanPipelineStageFlags>,
        A1: Into<VulkanAccessFlags>,
        P2: Into<VulkanPipelineStageFlags>,
        A2: Into<VulkanAccessFlags>,
        F: Into<VulkanImageAspectFlags>,
    >(
        src_stage_mask: P1,
        src_access_mask: A1,
        dst_stage_mask: P2,
        dst_access_mask: A2,
        old_layout: VulkanImageLayout,
        new_layout: VulkanImageLayout,
        src_queue_family_index: u32,
        dst_queue_family_index: u32,
        image: &'a VulkanImage,
        aspect_mask: F,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> VulkanImageMemoryBarrier<'a> {
        VulkanImageMemoryBarrier {
            inner: VkImageMemoryBarrier2 {
                src_stage_mask: src_stage_mask.into(),
                src_access_mask: src_access_mask.into(),
                dst_stage_mask: dst_stage_mask.into(),
                dst_access_mask: dst_access_mask.into(),
                old_layout,
                new_layout,
                src_queue_family_index,
                dst_queue_family_index,
                image: image.handle(),
                subresource_range: VkImageSubresourceRange {
                    aspect_mask: aspect_mask.into(),
                    base_mip_level,
                    level_count,
                    base_array_layer,
                    layer_count,
                },
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
