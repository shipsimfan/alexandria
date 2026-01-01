use vulkan::{VkGetPhysicalDeviceMemoryProperties, VkGetPhysicalDeviceProperties};

mod load;

/// The functions that are used by adapters associated with an instance
pub(in crate::instance) struct GraphicsAdapterFunctions {
    /// The function used to get information about an adapter
    pub get_physical_device_properties: VkGetPhysicalDeviceProperties,

    /// The function used to get information about an adapter's memory
    pub get_physical_device_memory_properties: VkGetPhysicalDeviceMemoryProperties,
}
