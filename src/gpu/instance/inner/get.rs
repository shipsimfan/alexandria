use crate::gpu::instance::VulkanInstanceInner;
use vulkan::VkInstance;

impl VulkanInstanceInner {
    /// Get accesss to the instance handle
    pub(in crate::gpu::instance) fn handle(&self) -> VkInstance {
        self.handle
    }
}
