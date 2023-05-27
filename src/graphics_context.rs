use crate::{create_error, Device, Instance, Result, Window};
use std::sync::Arc;

pub struct GraphicsContext {
    device: Arc<vulkan::Device>,
    instance: Arc<Instance>,

    graphics_queue: vulkan::Queue,
    swapchain: Arc<vulkan::Swapchain>,
    swapchain_images: Vec<vulkan::Image>,

    #[allow(unused)]
    window_ref_count: Arc<()>,
}

impl GraphicsContext {
    pub(crate) fn new(
        window: &mut Window,
        window_ref_count: Arc<()>,
        device: Device,
    ) -> Result<Self> {
        let (device, instance, queue_families, swapchain_settings) = device.consume();

        let device = device
            .create_device(vulkan::DeviceCreateInfo {
                queue_create_infos: vec![vulkan::DeviceQueueCreateInfo {
                    family_index: queue_families.graphics(),
                    priorities: vec![1.0],
                }],
                enabled_extensions: vec!["VK_KHR_swapchain".to_owned()],
                ..Default::default()
            })
            .map_err(|error| create_error!(DeviceCreationFailed, Some(Vulkan(error))))?;

        let graphics_queue = device.get_queue(queue_families.graphics(), 0);

        let swapchain = device
            .create_swapchain(vulkan::SwapchainCreateInfo {
                surface: window.surface(),
                min_image_count: swapchain_settings.image_count,
                image_format: swapchain_settings.format.format,
                image_color_space: swapchain_settings.format.color_space,
                image_extent: swapchain_settings.swap_extent,
                image_array_layers: 1,
                image_usage: vulkan::ImageUsageFlags::new(&[
                    vulkan::ImageUsageFlagBits::ColorAttachment,
                ]),
                image_sharing_mode: vulkan::SharingMode::Exclusive,
                queue_family_indices: vec![queue_families.graphics()],
                pre_transform: swapchain_settings.current_transform,
                composite_alpha: vulkan::CompositeAlphaFlagBits::Opaque,
                present_mode: swapchain_settings.present_mode,
                clipped: true,
                old_swapchain: None,
            })
            .map_err(|error| create_error!(SwapchainCreationFailed, Some(Vulkan(error))))?;

        let swapchain_images = swapchain
            .get_images()
            .map_err(|error| create_error!(SwapchainCreationFailed, Some(Vulkan(error))))?;

        Ok(GraphicsContext {
            device,
            instance,

            graphics_queue,
            swapchain,
            swapchain_images,

            window_ref_count,
        })
    }
}
