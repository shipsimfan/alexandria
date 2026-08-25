use crate::{
    gpu::{VulkanImageAspectFlags, VulkanImageBlit},
    math::Vector3i,
};

impl VulkanImageBlit {
    /// Get the source aspect mask for the image blit region
    pub fn src_aspect_mask(&self) -> VulkanImageAspectFlags {
        self.inner.src_subresource.aspect_mask
    }

    /// Get the source mip level for the image blit region
    pub fn src_mip_level(&self) -> u32 {
        self.inner.src_subresource.mip_level
    }

    /// Get the source base array layer for the image blit region
    pub fn src_base_array_layer(&self) -> u32 {
        self.inner.src_subresource.base_array_layer
    }

    /// Get the source layer count for the image blit region
    pub fn src_layer_count(&self) -> u32 {
        self.inner.src_subresource.layer_count
    }

    /// Get the first corner of the source image region to blit
    pub fn src_offsets(&self) -> Vector3i {
        self.inner.src_offsets[0].into()
    }

    /// Get the second corner of the source image region to blit
    pub fn src_offsets_end(&self) -> Vector3i {
        self.inner.src_offsets[1].into()
    }

    /// Get the destination aspect mask for the image blit region
    pub fn dst_aspect_mask(&self) -> VulkanImageAspectFlags {
        self.inner.dst_subresource.aspect_mask
    }

    /// Get the destination mip level for the image blit region
    pub fn dst_mip_level(&self) -> u32 {
        self.inner.dst_subresource.mip_level
    }

    /// Get the destination base array layer for the image blit region
    pub fn dst_base_array_layer(&self) -> u32 {
        self.inner.dst_subresource.base_array_layer
    }

    /// Get the destination layer count for the image blit region
    pub fn dst_layer_count(&self) -> u32 {
        self.inner.dst_subresource.layer_count
    }

    /// Get the first corner of the destination image region to blit
    pub fn dst_offsets(&self) -> Vector3i {
        self.inner.dst_offsets[0].into()
    }

    /// Get the second corner of the destination image region to blit
    pub fn dst_offsets_end(&self) -> Vector3i {
        self.inner.dst_offsets[1].into()
    }
}
