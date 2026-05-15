mod default;
mod set;
mod to_vk;

/// Vulkan 1.3 device features
pub struct VulkanDeviceVulkan13Features {
    /// `synchronization2` indicates whether the implementation supports the new set of
    /// synchronization commands introduced in `khr_synchronization2`.
    pub synchorization2: bool,

    /// `dynamic_rendering` specifies that the implementation supports dynamic render pass
    /// instances using the [`VulkanCommandBuffer::cmd_begin_rendering`] command.
    pub dynamic_rendering: bool,
}
