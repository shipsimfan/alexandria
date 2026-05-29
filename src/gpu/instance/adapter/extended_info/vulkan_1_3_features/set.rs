use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::{VK_FALSE, VK_TRUE};

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceVulkan13Features {
    /// Enable the new set of synchronization commands introduced in `khr_synchronization2`.
    pub fn enable_synchronization2(mut self) -> Self {
        self.inner.synchronization2 = VK_TRUE;
        self
    }

    /// Disable the new set of synchronization commands introduced in `khr_synchronization2`.
    pub fn disable_synchronization2(mut self) -> Self {
        self.inner.synchronization2 = VK_FALSE;
        self
    }

    /// Enable dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command.
    pub fn enable_dynamic_rendering(mut self) -> Self {
        self.inner.dynamic_rendering = VK_TRUE;
        self
    }

    /// Disable dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command.
    pub fn disable_dynamic_rendering(mut self) -> Self {
        self.inner.dynamic_rendering = VK_FALSE;
        self
    }
}
