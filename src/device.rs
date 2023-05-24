use crate::Instance;
use std::sync::Arc;
use vulkan::{
    VkColorSpaceKHR, VkExtent2D, VkFormat, VkPhysicalDevice, VkPresentModeKHR, VkQueueFlagBits,
    VkSurfaceFormatKHR, VkSurfaceKHR, VkSurfaceTransformFlagBitsKHR,
    VK_KHR_SWAPCHAIN_EXTENSION_NAME,
};

pub use vulkan::{VkPhysicalDeviceType as DeviceType, VK_UUID_SIZE as DEVICE_UUID_SIZE};

pub struct Device {
    inner: VkPhysicalDevice,
    instance: Arc<Instance>,

    name: String,
    id: (u32, u32),
    uuid: [u8; DEVICE_UUID_SIZE],
    r#type: DeviceType,

    graphics_queue_family_index: u32,
    surface_format: VkSurfaceFormatKHR,
    present_mode: VkPresentModeKHR,
    swap_extent: VkExtent2D,
    image_count: u32,
    surface_transform: VkSurfaceTransformFlagBitsKHR,
}

fn query_extensions(device: &VkPhysicalDevice) -> bool {
    for extension in match device.enumerate_device_extension_properties(None) {
        Ok(extensions) => extensions,
        Err(_) => return false,
    } {
        if extension.extension_name() == VK_KHR_SWAPCHAIN_EXTENSION_NAME {
            return true;
        }
    }

    false
}

fn query_queue_families(device: &VkPhysicalDevice, surface: &VkSurfaceKHR) -> Option<u32> {
    let queue_family_properties = device.get_queue_family_properties();
    for i in 0..queue_family_properties.len() {
        if !queue_family_properties[i]
            .flags()
            .contains(VkQueueFlagBits::Graphics)
        {
            continue;
        }

        if surface
            .get_physical_device_surface_support(&device, i as u32)
            .ok()?
        {
            return Some(i as u32);
        }
    }

    None
}

fn query_format(device: &VkPhysicalDevice, surface: &VkSurfaceKHR) -> Option<VkSurfaceFormatKHR> {
    let mut formats = surface.get_physical_device_surface_formats(device).ok()?;
    if formats.len() == 0 {
        return None;
    }

    for i in 0..formats.len() {
        if formats[i].format() == VkFormat::B8G8R8A8SRGB
            && formats[i].color_space() == VkColorSpaceKHR::SRGBNonLinear
        {
            return Some(formats.swap_remove(i));
        }
    }

    Some(formats.swap_remove(0))
}

fn query_present_mode(
    device: &VkPhysicalDevice,
    surface: &VkSurfaceKHR,
) -> Option<VkPresentModeKHR> {
    let present_modes = surface
        .get_physical_device_surface_present_modes(device)
        .ok()?;
    if present_modes.len() == 0 {
        return None;
    }

    for present_mode in present_modes {
        if present_mode == VkPresentModeKHR::Mailbox {
            return Some(present_mode);
        }
    }

    Some(VkPresentModeKHR::FIFO)
}

fn query_capabilities(
    device: &VkPhysicalDevice,
    surface: &VkSurfaceKHR,
) -> Option<(VkExtent2D, u32, VkSurfaceTransformFlagBitsKHR)> {
    let capabilities = surface
        .get_physical_device_surface_capabilities(&device)
        .ok()?;

    if capabilities.current_extent().width() != u32::MAX {
        Some((
            capabilities.current_extent().clone(),
            if capabilities.max_image_count() > 0
                && capabilities.min_image_count() + 1 > capabilities.max_image_count()
            {
                capabilities.max_image_count()
            } else {
                capabilities.min_image_count() + 1
            },
            capabilities.current_transform(),
        ))
    } else {
        None
    }
}

impl Device {
    pub(crate) fn new(
        inner: VkPhysicalDevice,
        surface: &VkSurfaceKHR,
        instance: Arc<Instance>,
    ) -> Option<Self> {
        // Get the basic information
        let properties = inner.get_properties();
        let name = properties.device_name().to_string_lossy().to_string();
        let id = (properties.vendor_id(), properties.device_id());
        let uuid = *properties.pipeline_cache_uuid();
        let r#type = properties.device_type();

        // Query compatibility
        if !query_extensions(&inner) {
            return None;
        }

        let surface_format = query_format(&inner, surface)?;
        let present_mode = query_present_mode(&inner, surface)?;
        let (swap_extent, image_count, surface_transform) = query_capabilities(&inner, surface)?;

        if let Some(graphics_queue_family_index) = query_queue_families(&inner, surface) {
            Some(Device {
                inner,
                instance,

                name,
                id,
                uuid,
                r#type,

                graphics_queue_family_index,
                surface_format,
                present_mode,
                swap_extent,
                image_count,
                surface_transform,
            })
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> (u32, u32) {
        self.id
    }

    pub fn uuid(&self) -> [u8; DEVICE_UUID_SIZE] {
        self.uuid
    }

    pub fn r#type(&self) -> DeviceType {
        self.r#type
    }

    pub(crate) fn inner(&self) -> &VkPhysicalDevice {
        &self.inner
    }

    pub(crate) fn graphics_queue_family_index(&self) -> u32 {
        self.graphics_queue_family_index
    }

    pub(crate) fn surface_format(&self) -> &VkSurfaceFormatKHR {
        &self.surface_format
    }

    pub(crate) fn present_mode(&self) -> VkPresentModeKHR {
        self.present_mode
    }

    pub(crate) fn swap_extent(&self) -> &VkExtent2D {
        &self.swap_extent
    }

    pub(crate) fn image_count(&self) -> u32 {
        self.image_count
    }

    pub(crate) fn surface_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
        self.surface_transform
    }
}
