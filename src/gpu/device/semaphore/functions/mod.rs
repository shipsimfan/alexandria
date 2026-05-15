use crate::FunctionSymbol;
use vulkan::{VkCreateSemaphore, VkDestroySemaphore};

mod load;

/// The functions that are used by semaphores associated with a device
pub(in crate::gpu::device) struct VulkanSemaphoreFunctions {
    /// The function to create a semaphore
    pub create_semaphore: FunctionSymbol<VkCreateSemaphore>,

    /// The function to destroy a semaphore
    pub destroy_semaphore: FunctionSymbol<VkDestroySemaphore>,
}
