use crate::Instance;
use std::sync::Arc;
use vulkan::{VkPhysicalDevice, VkQueueFlagBits, VkSurfaceKHR};

pub use vulkan::{VkPhysicalDeviceType as DeviceType, VK_UUID_SIZE as DEVICE_UUID_SIZE};

pub struct Device {
    inner: VkPhysicalDevice,
    instance: Arc<Instance>,

    name: String,
    id: (u32, u32),
    uuid: [u8; DEVICE_UUID_SIZE],
    r#type: DeviceType,

    graphics_queue_family_index: u32,
}

impl Device {
    pub(crate) fn new(
        inner: VkPhysicalDevice,
        surface: &VkSurfaceKHR,
        instance: Arc<Instance>,
    ) -> Option<Self> {
        let properties = inner.get_properties();
        let name = properties.device_name().to_string_lossy().to_string();
        let id = (properties.vendor_id(), properties.device_id());
        let uuid = *properties.pipeline_cache_uuid();
        let r#type = properties.device_type();

        let queue_family_properties = inner.get_queue_family_properties();
        let mut graphics_queue_family_index = None;
        for i in 0..queue_family_properties.len() {
            if queue_family_properties[i]
                .flags()
                .contains(VkQueueFlagBits::Graphics)
            {
                if surface
                    .get_physical_device_surface_support(&inner, i as u32)
                    .unwrap()
                {
                    graphics_queue_family_index = Some(i as u32);
                    break;
                }
            }
        }

        graphics_queue_family_index.map(|graphics_queue_family_index| Device {
            inner,
            instance,
            name,
            id,
            uuid,
            r#type,
            graphics_queue_family_index,
        })
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
}
