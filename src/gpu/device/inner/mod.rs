use crate::gpu::VulkanInstance;
use vulkan::{VkDevice, VkPhysicalDevice};

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::VulkanDeviceFunctions;

/// An interface for interacting with a specific Vulkan device
pub(in crate::gpu::device) struct VulkanDeviceInner {
    /// A handle to the underlying graphics device
    handle: VkDevice,

    /// A handle to the physical device that this logical device represents
    physical_device: VkPhysicalDevice,

    /// A reference to the instance that produced this device
    instance: VulkanInstance,

    /// The functions loaded for this device
    pub(in crate::gpu::device) functions: VulkanDeviceFunctions,
}
