use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanCommandBufferDebugUtilFunctions, load_device_function},
};
use vulkan::{
    VkDevice,
    ext_debug_utils::{VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT, VK_CMD_END_DEBUG_UTILS_LABEL_EXT},
};

impl VulkanCommandBufferDebugUtilFunctions {
    /// Load all the required command buffer functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanCommandBufferDebugUtilFunctions> {
        Ok(VulkanCommandBufferDebugUtilFunctions {
            cmd_begin_debug_utils_label: load_device_function!(
                instance,
                device,
                VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT
            )?,
            cmd_end_debug_utils_label: load_device_function!(
                instance,
                device,
                VK_CMD_END_DEBUG_UTILS_LABEL_EXT
            )?,
        })
    }
}
