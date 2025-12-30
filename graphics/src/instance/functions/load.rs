use crate::{Result, instance::GraphicsInstanceFunctions, util::load_instance_function};
use vulkan::{VK_DESTROY_INSTANCE, VkInstance};

impl GraphicsInstanceFunctions {
    /// Load all the required instance functions
    pub fn load(instance: VkInstance) -> Result<GraphicsInstanceFunctions> {
        Ok(GraphicsInstanceFunctions {
            destroy_instance: load_instance_function!(instance, VK_DESTROY_INSTANCE)?,
        })
    }
}
