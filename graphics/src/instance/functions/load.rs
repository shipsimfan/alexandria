use crate::{
    Result,
    instance::{GraphicsAdapterFunctions, GraphicsInstanceFunctions},
    util::load_instance_function,
};
use vulkan::{VK_DESTROY_INSTANCE, VK_ENUMERATE_PHYSICAL_DEVICES, VkInstance};

impl GraphicsInstanceFunctions {
    /// Load all the required instance functions
    pub fn load(instance: VkInstance) -> Result<GraphicsInstanceFunctions> {
        Ok(GraphicsInstanceFunctions {
            adapter: GraphicsAdapterFunctions::load(instance)?,

            enumerate_physical_devices: load_instance_function!(
                instance,
                VK_ENUMERATE_PHYSICAL_DEVICES
            )?,
            destroy_instance: load_instance_function!(instance, VK_DESTROY_INSTANCE)?,
        })
    }
}
