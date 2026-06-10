use crate::FunctionSymbol;
use vulkan::{
    VkEnumerateDeviceExtensionProperties, VkGetPhysicalDeviceFeatures2,
    VkGetPhysicalDeviceMemoryProperties, VkGetPhysicalDeviceProperties,
    VkGetPhysicalDeviceQueueFamilyProperties,
    khr_surface::VkGetPhysicalDeviceSurfaceCapabilitiesKhr,
};

mod load;

/// The functions that are used by adapters associated with an instance
pub(in crate::gpu) struct VulkanAdapterFunctions {
    /// The function used to get information about an adapter
    pub get_physical_device_properties: FunctionSymbol<VkGetPhysicalDeviceProperties>,

    /// The function used to get information about an adapter's memory
    pub get_physical_device_memory_properties: FunctionSymbol<VkGetPhysicalDeviceMemoryProperties>,

    /// The function used to get information about queues on an adapter
    pub get_physical_device_queue_family_properties:
        FunctionSymbol<VkGetPhysicalDeviceQueueFamilyProperties>,

    /// The function used the get the Vulkan extensions supported by an adapter
    pub enumerate_device_extension_properties: FunctionSymbol<VkEnumerateDeviceExtensionProperties>,

    /// The function used to get extended information about an adapter
    pub get_physical_device_features2: FunctionSymbol<VkGetPhysicalDeviceFeatures2>,

    /// The function used to get information about the surface capabilities of an adapter
    pub get_physical_device_surface_capabilities_khr:
        FunctionSymbol<VkGetPhysicalDeviceSurfaceCapabilitiesKhr>,
}
