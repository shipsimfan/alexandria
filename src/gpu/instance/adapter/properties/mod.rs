use vulkan::VkPhysicalDeviceProperties;

mod get;
mod new;

/// The properties of a Vulkan adapter
pub struct VulkanAdapterProperties {
    /// The raw Vulkan properties of the adapter
    inner: VkPhysicalDeviceProperties,
}
