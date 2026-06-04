use crate::gpu::{
    VulkanAccessFlags, VulkanBuffer, VulkanBufferMemoryBarrier, VulkanPipelineStageFlags,
};

impl<'a> VulkanBufferMemoryBarrier<'a> {
    /// Set the source pipeline stage mask of this memory barrier
    pub fn set_src_stage_mask<P: Into<VulkanPipelineStageFlags>>(&mut self, src_stage_mask: P) {
        self.inner.src_stage_mask = src_stage_mask.into();
    }

    /// Set the source access mask of this memory barrier
    pub fn set_src_access_mask<A: Into<VulkanAccessFlags>>(&mut self, src_access_mask: A) {
        self.inner.src_access_mask = src_access_mask.into();
    }

    /// Set the destination pipeline stage mask of this memory barrier
    pub fn set_dst_stage_mask<P: Into<VulkanPipelineStageFlags>>(&mut self, dst_stage_mask: P) {
        self.inner.dst_stage_mask = dst_stage_mask.into();
    }

    /// Set the destination access mask of this memory barrier
    pub fn set_dst_access_mask<A: Into<VulkanAccessFlags>>(&mut self, dst_access_mask: A) {
        self.inner.dst_access_mask = dst_access_mask.into();
    }

    /// Set the source queue family index of this memory barrier
    pub fn set_src_queue_family_index(&mut self, src_queue_family_index: u32) {
        self.inner.src_queue_family_index = src_queue_family_index;
    }

    /// Set the destination queue family index of this memory barrier
    pub fn set_dst_queue_family_index(&mut self, dst_queue_family_index: u32) {
        self.inner.dst_queue_family_index = dst_queue_family_index;
    }

    /// Set the buffer of this memory barrier
    pub fn set_buffer(&mut self, buffer: &'a VulkanBuffer) {
        self.inner.buffer = buffer.handle();
    }

    /// Set the offset into the buffer region affected by this memory barrier
    pub fn set_offset(&mut self, offset: u64) {
        self.inner.offset = offset;
    }

    /// Set the size of the buffer region affected by this memory barrier
    pub fn set_size(&mut self, size: u64) {
        self.inner.size = size;
    }
}
