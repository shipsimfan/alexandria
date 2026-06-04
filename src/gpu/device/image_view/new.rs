use crate::{
    Error, Result,
    gpu::{
        VulkanComponentMapping, VulkanFormat, VulkanImage, VulkanImageAspectFlags, VulkanImageView,
        VulkanImageViewCreateFlags, VulkanImageViewType,
    },
};
use std::ptr::null;
use vulkan::{VkImageSubresourceRange, VkImageView, VkImageViewCreateInfo, try_vulkan};

impl VulkanImageView {
    /// Create a new [`VulkanImageView`] for the given image and format
    pub(in crate::gpu::device) fn new(
        flags: VulkanImageViewCreateFlags,
        image: &VulkanImage,
        view_type: VulkanImageViewType,
        format: VulkanFormat,
        components: VulkanComponentMapping,
        aspect_mask: VulkanImageAspectFlags,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Result<VulkanImageView> {
        let creation_info = VkImageViewCreateInfo {
            flags,
            image: image.handle(),
            view_type,
            format,
            components,
            subresource_range: VkImageSubresourceRange {
                aspect_mask,
                base_mip_level,
                level_count,
                base_array_layer,
                layer_count,
            },
            ..Default::default()
        };

        let mut handle = VkImageView::null();
        try_vulkan!((image.device().functions().image_view.create_image_view)(
            image.device().handle(),
            &creation_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create an image view", vk))?;

        Ok(VulkanImageView {
            handle,
            image: image.clone(),
        })
    }
}
