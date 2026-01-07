use crate::{Result, instance::GraphicsDebugMessengerFunctions, util::load_instance_function};
use vulkan::{
    VkInstance,
    ext_debug_utils::{VK_CREATE_DEBUG_UTILS_MESSENGER_EXT, VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT},
};

impl GraphicsDebugMessengerFunctions {
    /// Load all the required debug messenger functions
    pub fn load(instance: VkInstance) -> Result<GraphicsDebugMessengerFunctions> {
        Ok(GraphicsDebugMessengerFunctions {
            create_debug_messenger: load_instance_function!(
                instance,
                VK_CREATE_DEBUG_UTILS_MESSENGER_EXT
            )?,
            destroy_debug_messenger: load_instance_function!(
                instance,
                VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT
            )?,
        })
    }
}
