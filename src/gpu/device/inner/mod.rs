use crate::gpu::VulkanInstance;
use vulkan::VkDevice;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::VulkanDeviceFunctions;

/// An interface for interacting with a specific Vulkan device
pub(in crate::gpu::device) struct VulkanDeviceInner {
    /// A handle to the underlying graphics device
    handle: VkDevice,

    /// A reference to the instance that produced this device
    _instance: VulkanInstance,

    /// The functions loaded for this device
    pub(in crate::gpu::device) functions: VulkanDeviceFunctions,
}
