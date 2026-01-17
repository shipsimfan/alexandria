use vulkan::{
    VkEnumerateDeviceExtensionProperties, VkGetPhysicalDeviceMemoryProperties,
    VkGetPhysicalDeviceProperties, VkGetPhysicalDeviceQueueFamilyProperties,
};

mod load;

/// The functions that are used by adapters associated with an instance
pub(in crate::instance) struct GraphicsAdapterFunctions {
    /// The function used to get information about an adapter
    pub get_physical_device_properties: VkGetPhysicalDeviceProperties,

    /// The function used to get information about an adapter's memory
    pub get_physical_device_memory_properties: VkGetPhysicalDeviceMemoryProperties,

    /// The function used to get information about queues on an adapter
    pub get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties,

    /// The function used the get the Vulkan extensions supported by an adapter
    pub enumerate_device_extension_properties: VkEnumerateDeviceExtensionProperties,
}
