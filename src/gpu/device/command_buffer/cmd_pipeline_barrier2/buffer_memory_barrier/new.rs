use crate::gpu::{
    VulkanAccessFlags, VulkanBuffer, VulkanBufferMemoryBarrier, VulkanPipelineStageFlags,
};
use std::marker::PhantomData;
use vulkan::VkBufferMemoryBarrier2;

impl<'a> VulkanBufferMemoryBarrier<'a> {
    /// Create a new [`VulkanBufferMemoryBarrier`]
    pub fn new<
        P1: Into<VulkanPipelineStageFlags>,
        A1: Into<VulkanAccessFlags>,
        P2: Into<VulkanPipelineStageFlags>,
        A2: Into<VulkanAccessFlags>,
    >(
        src_stage_mask: P1,
        src_access_mask: A1,
        dst_stage_mask: P2,
        dst_access_mask: A2,
        src_queue_family_index: u32,
        dst_queue_family_index: u32,
        buffer: &'a VulkanBuffer,
        offset: u64,
        size: u64,
    ) -> VulkanBufferMemoryBarrier<'a> {
        VulkanBufferMemoryBarrier {
            inner: VkBufferMemoryBarrier2 {
                src_stage_mask: src_stage_mask.into(),
                src_access_mask: src_access_mask.into(),
                dst_stage_mask: dst_stage_mask.into(),
                dst_access_mask: dst_access_mask.into(),
                src_queue_family_index,
                dst_queue_family_index,
                buffer: buffer.handle(),
                offset,
                size,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
