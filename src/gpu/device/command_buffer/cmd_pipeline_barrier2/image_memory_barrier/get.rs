use crate::gpu::{
    VulkanAccessFlags, VulkanImageAspectFlags, VulkanImageLayout, VulkanImageMemoryBarrier,
    VulkanPipelineStageFlags,
};

impl<'a> VulkanImageMemoryBarrier<'a> {
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

    /// Get the old image layout of this memory barrier
    pub fn old_layout(&self) -> VulkanImageLayout {
        self.inner.old_layout
    }

    /// Get the new image layout of this memory barrier
    pub fn new_layout(&self) -> VulkanImageLayout {
        self.inner.new_layout
    }

    /// Get the source queue family index of this memory barrier
    pub fn src_queue_family_index(&self) -> u32 {
        self.inner.src_queue_family_index
    }

    /// Get the destination queue family index of this memory barrier
    pub fn dst_queue_family_index(&self) -> u32 {
        self.inner.dst_queue_family_index
    }

    /// Get the aspect mask of this memory barrier
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        self.inner.subresource_range.aspect_mask
    }

    /// Get the base mip level of this memory barrier
    pub fn base_mip_level(&self) -> u32 {
        self.inner.subresource_range.base_mip_level
    }

    /// Get the level count of this memory barrier
    pub fn level_count(&self) -> u32 {
        self.inner.subresource_range.level_count
    }

    /// Get the base array layer of this memory barrier
    pub fn base_array_layer(&self) -> u32 {
        self.inner.subresource_range.base_array_layer
    }

    /// Get the layer count of this memory barrier
    pub fn layer_count(&self) -> u32 {
        self.inner.subresource_range.layer_count
    }
}
