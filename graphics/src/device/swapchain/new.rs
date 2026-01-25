use crate::{
    GraphicsError, Result, Swapchain, SwapchainFormat, SwapchainPresentMode, WindowSurface,
    device::GraphicsDeviceInner,
};
use alexandria_math::Vector2u;
use std::{ptr::null, sync::Arc};
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

        Ok(Swapchain {
            handle,
            _surface: surface,
            device,
        })
    }
}
