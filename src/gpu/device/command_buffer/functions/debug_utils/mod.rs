use crate::FunctionSymbol;
use vulkan::ext_debug_utils::{VkCmdBeginDebugUtilsLabelExt, VkCmdEndDebugUtilsLabelExt};

mod load;

/// The functions that are used by command buffers associated with a device when debugging utilities are needed
pub(in crate::gpu::device) struct VulkanCommandBufferDebugUtilFunctions {
    /// The function to begin a debug utils label in a command buffer
    pub cmd_begin_debug_utils_label: FunctionSymbol<VkCmdBeginDebugUtilsLabelExt>,

    /// The function to end a debug utils label in a command buffer
    pub cmd_end_debug_utils_label: FunctionSymbol<VkCmdEndDebugUtilsLabelExt>,
}
