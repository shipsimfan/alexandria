use crate::{
    Error, Result,
    gpu::{VulkanFormat, VulkanImage, VulkanImageView},
};
use std::ptr::null;
use vulkan::{
    VkImageAspectFlag, VkImageSubresourceRange, VkImageView, VkImageViewCreateInfo,
    VkImageViewType, try_vulkan,
};

impl VulkanImageView {
    /// Create a new [`VulkanImageView`] for the given image and format
    pub(in crate::gpu::device) fn new(
        image: &VulkanImage,
        format: VulkanFormat,
    ) -> Result<VulkanImageView> {
        let creation_info = VkImageViewCreateInfo {
            image: image.handle(),
            view_type: VkImageViewType::_2d,
            format: format.into_vk(),
            subresource_range: VkImageSubresourceRange {
                aspect_mask: VkImageAspectFlag::ColorBit.into(),
                level_count: 1,
                layer_count: 1,
                ..Default::default()
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
