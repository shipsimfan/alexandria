use crate::{
    Result,
    gpu::{
        VulkanComponentMapping, VulkanFormat, VulkanImage, VulkanImageAspectFlags, VulkanImageView,
        VulkanImageViewCreateFlags, VulkanImageViewType,
    },
};

impl VulkanImage {
    /// Create a new image view for this image
    pub fn create_image_view<
        F1: Into<VulkanImageViewCreateFlags>,
        F2: Into<VulkanImageAspectFlags>,
    >(
        &self,
        flags: F1,
        view_type: VulkanImageViewType,
        format: VulkanFormat,
        components: VulkanComponentMapping,
        aspect_mask: F2,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Result<VulkanImageView> {
        VulkanImageView::new(
            flags.into(),
            self,
            view_type,
            format,
            components,
            aspect_mask.into(),
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        )
    }
}
