use crate::gpu::{VulkanAccessFlags, VulkanMemoryBarrier, VulkanPipelineStageFlags};

impl VulkanMemoryBarrier {
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
}
