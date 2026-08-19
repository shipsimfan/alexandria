use crate::{
    Result,
    gpu::{
        VulkanDevice, VulkanFormat, VulkanImage, VulkanImageCreateFlags, VulkanImageLayout,
        VulkanImageTiling, VulkanImageType, VulkanImageUsageFlags, VulkanSampleCountFlag,
        VulkanSharingMode,
    },
    math::Vector3u,
};

impl VulkanDevice {
    /// Create a new [`VulkanImage`]
    pub fn create_image<F: Into<VulkanImageCreateFlags>, U: Into<VulkanImageUsageFlags>>(
        flags: F,
        image_type: VulkanImageType,
        format: VulkanFormat,
        extent: Vector3u,
        mip_levels: u32,
        array_layers: u32,
        samples: VulkanSampleCountFlag,
        tiling: VulkanImageTiling,
        usage: U,
        sharing_mode: VulkanSharingMode,
        queue_family_indices: &[u32],
        initial_layout: VulkanImageLayout,
        device: &VulkanDevice,
    ) -> Result<VulkanImage> {
        VulkanImage::new(
            flags.into(),
            image_type,
            format,
            extent,
            mip_levels,
            array_layers,
            samples,
            tiling,
            usage.into(),
            sharing_mode,
            queue_family_indices,
            initial_layout,
            device,
        )
    }
}
