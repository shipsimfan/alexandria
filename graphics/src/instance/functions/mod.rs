use crate::instance::GraphicsAdapterFunctions;
use vulkan::{VkDestroyInstance, VkEnumeratePhysicalDevices};

mod load;

/// The functions loaded for a specific graphics instance
pub(in crate::instance) struct GraphicsInstanceFunctions {
    /// The functions used by adapters
    pub adapter: GraphicsAdapterFunctions,

    /// The function used to enumerate the physical devices on the system
    pub enumerate_physical_devices: VkEnumeratePhysicalDevices,

    /// The function used to destroy the instance
    pub destroy_instance: VkDestroyInstance,
}
