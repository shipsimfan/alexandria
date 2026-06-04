use crate::{
    Error, Result,
    gpu::{
        VulkanColorSpace, VulkanCompositeAlphaFlag, VulkanDevice, VulkanFormat, VulkanImage,
        VulkanImageUsageFlags, VulkanPresentMode, VulkanSharingMode, VulkanSurface,
        VulkanSurfaceTransformFlag, VulkanSwapchain, VulkanSwapchainCreateFlags,
    },
    math::Vector2u,
};
use std::ptr::{null, null_mut};
use vulkan::{
    VK_FALSE, VK_TRUE,
    khr_swapchain::{VkSwapchainCreateInfoKhr, VkSwapchainKhr},
    try_vulkan,
};

impl<'surface> VulkanSwapchain<'surface> {
    /// Create a new [`VulkanSwapchain`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanSwapchainCreateFlags,
        surface: &'surface mut VulkanSurface,

        min_image_count: u32,
        image_format: VulkanFormat,
        image_color_space: VulkanColorSpace,
        image_size: Vector2u,
        image_array_layers: u32,
        image_usage: VulkanImageUsageFlags,
        image_sharing_mode: VulkanSharingMode,

        queue_family_indices: &[u32],

        pre_transform: VulkanSurfaceTransformFlag,
        composite_alpha: VulkanCompositeAlphaFlag,
        present_mode: VulkanPresentMode,
        clipped: bool,

        old_swapchain: Option<&mut VulkanSwapchain>,

        device: VulkanDevice,
    ) -> Result<VulkanSwapchain<'surface>> {
        // Create swapchain
        let create_info = VkSwapchainCreateInfoKhr {
            flags: flags.into(),
            surface: surface.handle(),
            min_image_count,
            image_format,
            image_color_space,
            image_extent: image_size.into(),
            image_array_layers,
            image_usage: image_usage.into(),
            image_sharing_mode,
            queue_family_index_count: queue_family_indices.len() as _,
            queue_family_indices: queue_family_indices.as_ptr(),
            pre_transform: pre_transform.into(),
            composite_alpha: composite_alpha.into(),
            present_mode,
            clipped: if clipped { VK_TRUE } else { VK_FALSE },
            old_swapchain: old_swapchain.map_or(VkSwapchainKhr::null(), |s| s.handle()),
            ..Default::default()
        };

        let mut handle = VkSwapchainKhr::null();
        try_vulkan!((device.functions().swapchain().create_swapchain)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create a swapchain", vk))?;

        // Get images
        let mut image_count = 0;
        try_vulkan!((device.functions().swapchain().get_swapchain_images)(
            device.handle(),
            handle,
            &mut image_count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get swapchain image count", vk))?;

        let mut images = Vec::with_capacity(image_count as _);
        try_vulkan!((device.functions().swapchain().get_swapchain_images)(
            device.handle(),
            handle,
            &mut image_count,
            images.as_mut_ptr()
        ))
        .map_err(|vk| Error::new_with("unable to get swapchain images", vk))?;
        unsafe { images.set_len(image_count as _) };

        let images = images
            .into_iter()
            .map(|handle| VulkanImage::from_handle(handle, device.clone()))
            .collect();

        Ok(VulkanSwapchain {
            handle,
            images,
            _surface: surface,
            device,
        })
    }
}
