use crate::{Result, instance::GraphicsAdapterFunctions, util::load_instance_function};
use vulkan::{
    VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES, VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES,
    VK_GET_PHYSICAL_DEVICE_PROPERTIES, VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES, VkInstance,
};

impl GraphicsAdapterFunctions {
    /// Load all the required adapter functions
    pub fn load(instance: VkInstance) -> Result<GraphicsAdapterFunctions> {
        Ok(GraphicsAdapterFunctions {
            get_physical_device_properties: load_instance_function!(
                instance,
                VK_GET_PHYSICAL_DEVICE_PROPERTIES
            )?,
            get_physical_device_memory_properties: load_instance_function!(
                instance,
                VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES
            )?,
            get_physical_device_queue_family_properties: load_instance_function!(
                instance,
                VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES
            )?,
            enumerate_device_extension_properties: load_instance_function!(
                instance,
                VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES
            )?,
        })
    }
}
