use crate::gpu::VulkanDeviceVulkan13Features;

impl VulkanDeviceVulkan13Features {
    /// Enable the new set of synchronization commands introduced in `khr_synchronization2`.
    pub fn synchronization2(mut self) -> Self {
        self.synchorization2 = true;
        self
    }

    /// Enable dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command.
    pub fn dynamic_rendering(mut self) -> Self {
        self.dynamic_rendering = true;
        self
    }
}
