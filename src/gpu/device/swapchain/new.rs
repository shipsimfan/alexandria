use crate::{
    Error, Result,
    gpu::{
        VulkanDevice, VulkanFormat, VulkanImage, VulkanSurface, VulkanSwapchain,
        VulkanSwapchainPresentMode,
    },
    math::Vector2i,
};
use std::ptr::{null, null_mut};
use vulkan::{
    VK_TRUE, VkExtent2D, VkImageUsageFlag, VkSharingMode,
    khr_surface::{VkCompositeAlphaFlagKhr, VkSurfaceTransformFlagKhr},
    khr_swapchain::{VkSwapchainCreateInfoKhr, VkSwapchainKhr},
    try_vulkan,
};

impl<'surface> VulkanSwapchain<'surface> {
    /// Create a new [`VulkanSwapchain`]
    pub(in crate::gpu::device) fn new(
        image_count: u32,
        image_format: VulkanFormat,
        image_size: Vector2i,
        present_mode: VulkanSwapchainPresentMode,

        surface: &'surface VulkanSurface,
        device: VulkanDevice,
    ) -> Result<VulkanSwapchain<'surface>> {
        // Create swapchain
        let (image_format, image_color_space) = image_format.into_vk_surface_format();

        let create_info = VkSwapchainCreateInfoKhr {
            surface: surface.handle(),
            min_image_count: image_count,
            image_format,
            image_color_space,
            image_extent: VkExtent2D {
                width: image_size.x as _,
                height: image_size.y as _,
            },
            image_array_layers: 1,
            image_usage: VkImageUsageFlag::ColorAttachment.into(),
            image_sharing_mode: VkSharingMode::Exclusive,
            pre_transform: VkSurfaceTransformFlagKhr::IdentityKhr.into(),
            composite_alpha: VkCompositeAlphaFlagKhr::OpaqueKhr.into(),
            present_mode: present_mode.into_vk(),
            clipped: VK_TRUE,
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
            .map(|handle| VulkanImage::new(handle, device.clone()))
            .collect();

        Ok(VulkanSwapchain {
            handle,
            images,
            _surface: surface,
            device,
        })
    }
}
