use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::VK_TRUE;

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceVulkan13Features {
    /// Get whether the new set of synchronization commands introduced in `khr_synchronization2` is enabled
    pub fn synchronization2(&self) -> bool {
        self.inner.synchronization2 == VK_TRUE
    }

    /// Get whether dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command is enabled
    pub fn dynamic_rendering(&self) -> bool {
        self.inner.dynamic_rendering == VK_TRUE
    }
}
