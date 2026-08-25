use crate::{
    gpu::{VulkanImageAspectFlags, VulkanImageBlit},
    math::Vector3i,
};
use vulkan::{VkImageBlit, VkImageSubresourceLayers};

impl VulkanImageBlit {
    /// Create a new [`VulkanImageBlit`]
    pub fn new<F1: Into<VulkanImageAspectFlags>, F2: Into<VulkanImageAspectFlags>>(
        src_aspect_mask: F1,
        src_mip_level: u32,
        src_base_array_layer: u32,
        src_layer_count: u32,
        src_offsets: [Vector3i; 2],
        dst_aspect_mask: F2,
        dst_mip_level: u32,
        dst_base_array_layer: u32,
        dst_layer_count: u32,
        dst_offsets: [Vector3i; 2],
    ) -> VulkanImageBlit {
        VulkanImageBlit {
            inner: VkImageBlit {
                src_subresource: VkImageSubresourceLayers {
                    aspect_mask: src_aspect_mask.into(),
                    mip_level: src_mip_level,
                    base_array_layer: src_base_array_layer,
                    layer_count: src_layer_count,
                },
                src_offsets: [src_offsets[0].into(), src_offsets[1].into()],
                dst_subresource: VkImageSubresourceLayers {
                    aspect_mask: dst_aspect_mask.into(),
                    mip_level: dst_mip_level,
                    base_array_layer: dst_base_array_layer,
                    layer_count: dst_layer_count,
                },
                dst_offsets: [dst_offsets[0].into(), dst_offsets[1].into()],
            },
        }
    }
}
