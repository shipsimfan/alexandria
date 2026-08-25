use crate::{
    gpu::{VulkanImageAspectFlags, VulkanImageBlit},
    math::Vector3i,
};

impl VulkanImageBlit {
    /// Set the source aspect mask for the image blit region
    pub fn set_src_aspect_mask<F: Into<VulkanImageAspectFlags>>(&mut self, aspect_mask: F) {
        self.inner.src_subresource.aspect_mask = aspect_mask.into();
    }

    /// Set the source mip level for the image blit region
    pub fn set_src_mip_level(&mut self, mip_level: u32) {
        self.inner.src_subresource.mip_level = mip_level;
    }

    /// Set the source base array layer for the image blit region
    pub fn set_src_base_array_layer(&mut self, base_array_layer: u32) {
        self.inner.src_subresource.base_array_layer = base_array_layer;
    }

    /// Set the source layer count for the image blit region
    pub fn set_src_layer_count(&mut self, layer_count: u32) {
        self.inner.src_subresource.layer_count = layer_count;
    }

    /// Set the corners of the source image region to blit
    pub fn set_src_offsets(&mut self, offsets: [Vector3i; 2]) {
        self.inner.src_offsets = [offsets[0].into(), offsets[1].into()];
    }

    /// Set the first corner of the source image region to blit
    pub fn set_src_offset_start(&mut self, offset: Vector3i) {
        self.inner.src_offsets[0] = offset.into();
    }

    /// Set the second corner of the source image region to blit
    pub fn set_src_offset_end(&mut self, offset: Vector3i) {
        self.inner.src_offsets[1] = offset.into();
    }

    /// Set the destination aspect mask for the image blit region
    pub fn set_dst_aspect_mask<F: Into<VulkanImageAspectFlags>>(&mut self, aspect_mask: F) {
        self.inner.dst_subresource.aspect_mask = aspect_mask.into();
    }

    /// Set the destination mip level for the image blit region
    pub fn set_dst_mip_level(&mut self, mip_level: u32) {
        self.inner.dst_subresource.mip_level = mip_level;
    }

    /// Set the destination base array layer for the image blit region
    pub fn set_dst_base_array_layer(&mut self, base_array_layer: u32) {
        self.inner.dst_subresource.base_array_layer = base_array_layer;
    }

    /// Set the destination layer count for the image blit region
    pub fn set_dst_layer_count(&mut self, layer_count: u32) {
        self.inner.dst_subresource.layer_count = layer_count;
    }

    /// Set the corners of the destination image region to blit
    pub fn set_dst_offsets(&mut self, offsets: [Vector3i; 2]) {
        self.inner.dst_offsets = [offsets[0].into(), offsets[1].into()];
    }

    /// Set the first corner of the destination image region to blit
    pub fn set_dst_offset_start(&mut self, offset: Vector3i) {
        self.inner.dst_offsets[0] = offset.into();
    }

    /// Set the second corner of the destination image region to blit
    pub fn set_dst_offset_end(&mut self, offset: Vector3i) {
        self.inner.dst_offsets[1] = offset.into();
    }
}
