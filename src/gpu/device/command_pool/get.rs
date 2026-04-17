use crate::gpu::{VulkanCommandPool, VulkanDevice};
use vulkan::VkCommandPool;

impl VulkanCommandPool {
    /// Get the handle and device of the underlying Vulkan command pool
    ///
    /// This needs to be a `with` style function because the command pool must be accessed
    /// synchronously, and therefore is behind a mutex.
    pub(in crate::gpu::device) fn with_handle_and_device<
        R,
        F: FnOnce(VkCommandPool, &VulkanDevice) -> R,
    >(
        &self,
        f: F,
    ) -> R {
        let inner = self.inner.lock().unwrap();
        f(inner.handle(), inner.device())
    }
}
