use crate::Instance;
use std::sync::Arc;
use vulkan::{
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkQueueFamilyProperties, VkQueueFlagBits,
};

pub use vulkan::VkPhysicalDeviceType as DeviceType;

pub struct Device {
    inner: VkPhysicalDevice,
    instance: Arc<Instance>,

    properties: VkPhysicalDeviceProperties,
    features: VkPhysicalDeviceFeatures,
    queue_family_properties: Vec<VkQueueFamilyProperties>,
}

impl Device {
    pub(crate) fn new(inner: VkPhysicalDevice, instance: Arc<Instance>) -> Self {
        let properties = inner.get_properties();
        let features = inner.get_features();
        let queue_family_properties = inner.get_queue_family_properties();

        Device {
            inner,
            instance,

            properties,
            features,
            queue_family_properties,
        }
    }

    pub fn name(&self) -> &str {
        self.properties.device_name().to_str().unwrap()
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

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub(crate) fn get_graphics_queue_index(&self) -> Option<usize> {
        for i in 0..self.queue_family_properties.len() {
            if self.queue_family_properties[i]
                .flags()
                .contains(VkQueueFlagBits::Graphics)
            {
                return Some(i);
            }
        }

        None
    }

    pub(crate) fn consume(self) -> (VkPhysicalDevice, Arc<Instance>) {
        (self.inner, self.instance)
    }
}
