use crate::{Result, instance::GraphicsAdapterFunctions, util::load_instance_function};
use vulkan::{VK_GET_PHYSICAL_DEVICE_PROPERTIES, VkInstance};

impl GraphicsAdapterFunctions {
    /// Load all the required adapter functions
    pub fn load(instance: VkInstance) -> Result<GraphicsAdapterFunctions> {
        Ok(GraphicsAdapterFunctions {
            get_physical_device_properties: load_instance_function!(
                instance,
                VK_GET_PHYSICAL_DEVICE_PROPERTIES
            )?,
        })
    }
}
