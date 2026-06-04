use crate::gpu::{
    VulkanAccessFlags, VulkanImage, VulkanImageAspectFlags, VulkanImageLayout,
    VulkanImageMemoryBarrier, VulkanPipelineStageFlags,
};

impl<'a> VulkanImageMemoryBarrier<'a> {
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

    /// Set the old image layout of this memory barrier
    pub fn set_old_layout(&mut self, old_layout: VulkanImageLayout) {
        self.inner.old_layout = old_layout;
    }

    /// Set the new image layout of this memory barrier
    pub fn set_new_layout(&mut self, new_layout: VulkanImageLayout) {
        self.inner.new_layout = new_layout;
    }

    /// Set the source queue family index of this memory barrier
    pub fn set_src_queue_family_index(&mut self, src_queue_family_index: u32) {
        self.inner.src_queue_family_index = src_queue_family_index;
    }

    /// Set the destination queue family index of this memory barrier
    pub fn set_dst_queue_family_index(&mut self, dst_queue_family_index: u32) {
        self.inner.dst_queue_family_index = dst_queue_family_index;
    }

    /// Set the image of this memory barrier
    pub fn set_image(&mut self, image: &'a VulkanImage) {
        self.inner.image = image.handle();
    }

    /// Set the aspect mask of this memory barrier
    pub fn set_aspect_mask<F: Into<VulkanImageAspectFlags>>(&mut self, aspect_mask: F) {
        self.inner.subresource_range.aspect_mask = aspect_mask.into();
    }

    /// Set the base mip level of this memory barrier
    pub fn set_base_mip_level(&mut self, base_mip_level: u32) {
        self.inner.subresource_range.base_mip_level = base_mip_level;
    }

    /// Set the level count of this memory barrier
    pub fn set_level_count(&mut self, level_count: u32) {
        self.inner.subresource_range.level_count = level_count;
    }

    /// Set the base array layer of this memory barrier
    pub fn set_base_array_layer(&mut self, base_array_layer: u32) {
        self.inner.subresource_range.base_array_layer = base_array_layer;
    }

    /// Set the layer count of this memory barrier
    pub fn set_layer_count(&mut self, layer_count: u32) {
        self.inner.subresource_range.layer_count = layer_count;
    }
}
