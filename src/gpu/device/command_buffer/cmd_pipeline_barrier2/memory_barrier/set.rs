use crate::gpu::{VulkanAccessFlags, VulkanMemoryBarrier, VulkanPipelineStageFlags};

impl VulkanMemoryBarrier {
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
}
