use crate::gpu::{VulkanInstanceFunctions, instance::VulkanInstanceInner};
use vulkan::VkInstance;

impl VulkanInstanceInner {
    /// Get accesss to the instance handle
    pub fn handle(&self) -> VkInstance {
        self.handle
    }

    /// Get the instance level functions
    pub fn functions(&self) -> &VulkanInstanceFunctions {
        &self.functions
    }
}
