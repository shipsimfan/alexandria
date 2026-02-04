use crate::FunctionSymbol;
use vulkan::{
    VkCreateInstance, VkEnumerateInstanceExtensionProperties, VkEnumerateInstanceLayerProperties,
    VkEnumerateInstanceVersion, VkGetInstanceProcAddr,
};

mod load;

/// The Vulkan functions not tied to a specific instance
pub(in crate::gpu::subsystem) struct GlobalVulkanFunctions {
    /// The function used to get other functions
    pub get_instance_proc_addr: FunctionSymbol<VkGetInstanceProcAddr>,

    /// The function used to enumerate the supported Vulkan version
    pub enumerate_instance_version: FunctionSymbol<VkEnumerateInstanceVersion>,

    /// The function used to enumerate supported instance extensions
    pub enumerate_instance_extensions: FunctionSymbol<VkEnumerateInstanceExtensionProperties>,

    /// The function used to enumerate supported instance layers
    pub enumerate_instance_layers: FunctionSymbol<VkEnumerateInstanceLayerProperties>,

    /// The function to create an instance
    pub create_instance: FunctionSymbol<VkCreateInstance>,
}
