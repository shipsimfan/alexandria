use crate::FunctionSymbol;
use vulkan::{
    VkEnumerateDeviceExtensionProperties, VkGetPhysicalDeviceMemoryProperties,
    VkGetPhysicalDeviceProperties, VkGetPhysicalDeviceQueueFamilyProperties,
};

mod load;

/// The functions that are used by adapters associated with an instance
pub(in crate::gpu::instance) struct VulkanAdapterFunctions {
    /// The function used to get information about an adapter
    pub get_physical_device_properties: FunctionSymbol<VkGetPhysicalDeviceProperties>,

    /// The function used to get information about an adapter's memory
    pub get_physical_device_memory_properties: FunctionSymbol<VkGetPhysicalDeviceMemoryProperties>,

    /// The function used to get information about queues on an adapter
    pub get_physical_device_queue_family_properties:
        FunctionSymbol<VkGetPhysicalDeviceQueueFamilyProperties>,

    /// The function used the get the Vulkan extensions supported by an adapter
    pub enumerate_device_extension_properties: FunctionSymbol<VkEnumerateDeviceExtensionProperties>,
}
