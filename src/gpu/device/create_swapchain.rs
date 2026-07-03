use crate::{
    Result,
    gpu::{
        VulkanColorSpace, VulkanCompositeAlphaFlag, VulkanDevice, VulkanFormat,
        VulkanImageUsageFlags, VulkanPresentMode, VulkanSharingMode, VulkanSurface,
        VulkanSurfaceTransformFlag, VulkanSwapchain, VulkanSwapchainCreateFlags,
    },
    math::Vector2u,
};

impl VulkanDevice {
    /// Create a new [`VulkanSwapchain`]
    pub fn create_swapchain<
        'surface,
        F1: Into<VulkanSwapchainCreateFlags>,
        F2: Into<VulkanImageUsageFlags>,
    >(
        &self,

        flags: F1,
        surface: &'surface mut VulkanSurface,

        min_image_count: u32,
        image_format: VulkanFormat,
        image_color_space: VulkanColorSpace,
        image_size: Vector2u,
        image_array_layers: u32,
        image_usage: F2,
        image_sharing_mode: VulkanSharingMode,

        queue_family_indices: &[u32],

        pre_transform: VulkanSurfaceTransformFlag,
        composite_alpha: VulkanCompositeAlphaFlag,
        present_mode: VulkanPresentMode,
        clipped: bool,

        old_swapchain: Option<&mut VulkanSwapchain>,
    ) -> Result<VulkanSwapchain<'surface>> {
        VulkanSwapchain::new(
            flags.into(),
            surface,
            min_image_count,
            image_format,
            image_color_space,
            image_size,
            image_array_layers,
            image_usage.into(),
            image_sharing_mode,
            queue_family_indices,
            pre_transform,
            composite_alpha,
            present_mode,
            clipped,
            old_swapchain,
            self,
        )
    }
}
