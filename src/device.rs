use vulkan::{
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkQueueFamilyProperties, VkQueueFlagBits,
};

pub use vulkan::VkPhysicalDeviceType as DeviceType;

pub struct Device {
    properties: VkPhysicalDeviceProperties,
    features: VkPhysicalDeviceFeatures,
    queue_family_properties: Vec<VkQueueFamilyProperties>,
}

impl Device {
    pub(crate) fn new(inner: VkPhysicalDevice) -> Self {
        let properties = inner.get_properties();
        let features = inner.get_features();
        let queue_family_properties = inner.get_queue_family_properties();

        Device {
            properties,
            features,
            queue_family_properties,
        }
    }

    pub fn name(&self) -> &str {
        self.properties.device_name()
    }

    pub fn id(&self) -> (u32, u32) {
        (self.properties.vendor_id(), self.properties.device_id())
    }

    pub fn uuid(&self) -> &[u8; 16] {
        self.properties.pipeline_cache_uuid()
    }

    pub fn r#type(&self) -> DeviceType {
        self.properties.device_type()
    }

    pub fn has_graphics_queues(&self) -> bool {
        for queue_family in &self.queue_family_properties {
            if queue_family.flags().contains(VkQueueFlagBits::Graphics) {
                return true;
            }
        }

        false
    }
}
