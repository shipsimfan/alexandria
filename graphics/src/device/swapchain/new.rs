use crate::{
    GpuImage, GraphicsError, Result, Swapchain, SwapchainFormat, SwapchainPresentMode,
    WindowSurface, device::GraphicsDeviceInner,
};
use alexandria_math::Vector2u;
use std::{
    ptr::{null, null_mut},
    sync::Arc,
};
use vulkan::{
    VK_TRUE, VkExtent2D, VkImageUsageFlag, VkSharingMode,
    khr_surface::{VkCompositeAlphaFlagKhr, VkSurfaceTransformFlagKhr},
    khr_swapchain::{VkSwapchainCreateInfoKhr, VkSwapchainKhr},
    try_vulkan,
};

impl<'surface> Swapchain<'surface> {
    /// Create a new [`Swapchain`]
    pub(in crate::device) fn new(
        image_count: u32,
        image_format: SwapchainFormat,
        image_size: Vector2u,
        present_mode: SwapchainPresentMode,

        surface: &'surface WindowSurface,
        device: Arc<GraphicsDeviceInner>,
    ) -> Result<Swapchain<'surface>> {
        // Create swapchain
        let (image_format, image_color_space) = image_format.into_vk();

        let create_info = VkSwapchainCreateInfoKhr {
            surface: surface.handle(),
            min_image_count: image_count,
            image_format,
            image_color_space,
            image_extent: VkExtent2D {
                width: image_size.x,
                height: image_size.y,
            },
            image_array_layers: 1,
            image_usage: VkImageUsageFlag::ColorAttachmentBit.into(),
            image_sharing_mode: VkSharingMode::Exclusive,
            pre_transform: VkSurfaceTransformFlagKhr::IdentityBitKhr.into(),
            composite_alpha: VkCompositeAlphaFlagKhr::OpaqueBitKhr.into(),
            present_mode: present_mode.into_vk(),
            clipped: VK_TRUE,
            ..Default::default()
        };

        let mut handle = VkSwapchainKhr::null();
        try_vulkan!((device.functions.swapchain().create_swapchain)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to create a swapchain", vk))?;

        // Get images
        let mut image_count = 0;
        try_vulkan!((device.functions.swapchain().get_swapchain_images)(
            device.handle(),
            handle,
            &mut image_count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain image count", vk))?;

        let mut images = Vec::with_capacity(image_count as _);
        try_vulkan!((device.functions.swapchain().get_swapchain_images)(
            device.handle(),
            handle,
            &mut image_count,
            images.as_mut_ptr()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain images", vk))?;
        unsafe { images.set_len(image_count as _) };

        let images = images
            .into_iter()
            .map(|handle| GpuImage::new(handle, device.clone()))
            .collect();

        Ok(Swapchain {
            handle,
            images,
            _surface: surface,
            device,
        })
    }
}
