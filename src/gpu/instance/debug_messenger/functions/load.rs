use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanDebugMessengerFunctions, util::load_instance_function},
};
use vulkan::{
    VkInstance,
    ext_debug_utils::{VK_CREATE_DEBUG_UTILS_MESSENGER_EXT, VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT},
};

impl VulkanDebugMessengerFunctions {
    /// Load all the required debug messenger functions
    pub fn load(
        context: &GpuSubsystem,
        instance: VkInstance,
    ) -> Result<VulkanDebugMessengerFunctions> {
        Ok(VulkanDebugMessengerFunctions {
            create_debug_messenger: load_instance_function!(
                context,
                instance,
                VK_CREATE_DEBUG_UTILS_MESSENGER_EXT
            )?,
            destroy_debug_messenger: load_instance_function!(
                context,
                instance,
                VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT
            )?,
        })
    }
}
