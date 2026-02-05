use crate::{
    FunctionSymbol,
    gpu::instance::{VulkanAdapterFunctions, VulkanDebugMessengerFunctions},
};
use vulkan::{VkCreateDevice, VkDestroyInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr};

mod get;
mod load;

/// The functions loaded for a specific graphics instance
pub(in crate::gpu) struct VulkanInstanceFunctions {
    /* Function Groups */
    /// The functions used by adapters
    pub(in crate::gpu::instance) adapter: VulkanAdapterFunctions,

    /// The functions used by debug messenger
    debug_messenger: Option<VulkanDebugMessengerFunctions>,

    /* Specific Functions */
    /// The function used to enumerate the physical devices on the system
    pub(in crate::gpu::instance) enumerate_physical_devices:
        FunctionSymbol<VkEnumeratePhysicalDevices>,

    /// The function used to destroy the instance
    pub(in crate::gpu::instance) destroy_instance: FunctionSymbol<VkDestroyInstance>,

    /// The function to create a graphics device
    pub create_device: FunctionSymbol<VkCreateDevice>,

    /// The function to get an address of graphics device procedure
    pub get_device_proc_addr: FunctionSymbol<VkGetDeviceProcAddr>,
}
