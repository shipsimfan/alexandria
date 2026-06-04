use crate::gpu::{VulkanAccessFlags, VulkanBufferMemoryBarrier, VulkanPipelineStageFlags};

impl<'a> VulkanBufferMemoryBarrier<'a> {
    /// Get the source pipeline stage mask of this memory barrier
    pub fn src_stage_mask(&self) -> VulkanPipelineStageFlags {
        self.inner.src_stage_mask
    }

    /// Get the source access mask of this memory barrier
    pub fn src_access_mask(&self) -> VulkanAccessFlags {
        self.inner.src_access_mask
    }

    /// Get the destination pipeline stage mask of this memory barrier
    pub fn dst_stage_mask(&self) -> VulkanPipelineStageFlags {
        self.inner.dst_stage_mask
    }

    /// Get the destination access mask of this memory barrier
    pub fn dst_access_mask(&self) -> VulkanAccessFlags {
        self.inner.dst_access_mask
    }

    /// Get the source queue family index of this memory barrier
    pub fn src_queue_family_index(&self) -> u32 {
        self.inner.src_queue_family_index
    }

    /// Get the destination queue family index of this memory barrier
    pub fn dst_queue_family_index(&self) -> u32 {
        self.inner.dst_queue_family_index
    }

    /// Get the offset into the buffer region affected by this memory barrier
    pub fn offset(&self) -> u64 {
        self.inner.offset
    }

    /// Get the size of the buffer region affected by this memory barrier
    pub fn size(&self) -> u64 {
        self.inner.size
    }
}
