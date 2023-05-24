use crate::{create_error, Device, Result};
use std::sync::Arc;
use vulkan::{
    VkCompositeAlphaFlagBitsKHR, VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateFlags,
    VkDeviceQueueCreateInfo, VkImage, VkImageUsageFlagBits, VkImageUsageFlags, VkQueue,
    VkSharingMode, VkSurfaceKHR, VkSwapchainCreateFlagsKHR, VkSwapchainCreateInfoKHR,
    VkSwapchainKHR, VK_KHR_SWAPCHAIN_EXTENSION_NAME,
};

pub struct GraphicsContext {
    device: Arc<VkDevice>,
    graphics_queue: VkQueue,
    swapchain_images: Vec<VkImage>,
    swapchain: VkSwapchainKHR,

    #[allow(unused)]
    window_ref_count: Arc<()>,
}

impl GraphicsContext {
    pub(crate) fn new(
        window_ref_count: Arc<()>,
        device: Device,
        surface: &VkSurfaceKHR,
    ) -> Result<Self> {
        let vk_device = device
            .inner()
            .create_device(&VkDeviceCreateInfo::new(
                &[VkDeviceQueueCreateInfo::new(
                    VkDeviceQueueCreateFlags::new(&[]),
                    device.graphics_queue_family_index(),
                    &[1.0],
                )],
                &[],
                &[VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr()],
                None,
            ))
            .map_err(|error| create_error!(GraphicsContextCreationFailed, Some(Vulkan(error))))?;

        let graphics_queue = vk_device.get_device_queue(device.graphics_queue_family_index(), 0);

        let swapchain = vk_device
            .create_swapchain(&VkSwapchainCreateInfoKHR::new(
                VkSwapchainCreateFlagsKHR::new(&[]),
                surface,
                device.image_count(),
                device.surface_format().format(),
                device.surface_format().color_space(),
                device.swap_extent().clone(),
                1,
                VkImageUsageFlags::new(&[VkImageUsageFlagBits::ColorAttachment]),
                VkSharingMode::Exclusive,
                &[],
                device.surface_transform(),
                VkCompositeAlphaFlagBitsKHR::Opaque,
                device.present_mode(),
                true,
                None,
            ))
            .map_err(|error| create_error!(GraphicsContextCreationFailed, Some(Vulkan(error))))?;

        let swapchain_images = swapchain
            .get_swapchain_images()
            .map_err(|error| create_error!(GraphicsContextCreationFailed, Some(Vulkan(error))))?;

        Ok(GraphicsContext {
            device: vk_device,
            graphics_queue,
            swapchain_images,
            swapchain,

            window_ref_count,
        })
    }
}
