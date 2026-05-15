use crate::FunctionSymbol;
use vulkan::{VkCreateFence, VkDestroyFence, VkResetFences, VkWaitForFences};

mod load;

/// The functions that are used by fences associated with a device
pub(in crate::gpu::device) struct VulkanFenceFunctions {
    /// The function to create a fence
    pub create_fence: FunctionSymbol<VkCreateFence>,

    /// The function to destroy a fence
    pub destroy_fence: FunctionSymbol<VkDestroyFence>,

    /// The function used to wait for multiple fences
    pub wait_for_fences: FunctionSymbol<VkWaitForFences>,

    /// The function used to reset multiple fences
    pub reset_fences: FunctionSymbol<VkResetFences>,
}
