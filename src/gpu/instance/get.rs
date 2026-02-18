use crate::gpu::{VulkanInstance, VulkanInstanceFunctions};
use vulkan::VkInstance;

impl VulkanInstance {
    /// Get accesss to the instance handle
    pub(in crate::gpu::instance) fn handle(&self) -> VkInstance {
        self.inner.handle()
    }

    /// Get the instance level functions
    pub(in crate::gpu::instance) fn functions(&self) -> &VulkanInstanceFunctions {
        self.inner.functions()
    }
}
