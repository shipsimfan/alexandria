#[cfg(target_os = "windows")]
use crate::gpu::instance::VulkanWin32SurfaceFunctions;
use crate::{
    FunctionSymbol,
    gpu::instance::{
        VulkanAdapterFunctions, VulkanDebugMessengerFunctions, VulkanSurfaceFunctions,
    },
};
use vulkan::{VkCreateDevice, VkDestroyInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr};

mod get;
mod load;

/// The functions loaded for a specific graphics instance
pub(in crate::gpu) struct VulkanInstanceFunctions {
    /** Function Groups **/

    /// The functions used by adapters
    pub(in crate::gpu::instance) adapter: VulkanAdapterFunctions,

    /// The functions used by debug messengers
    debug_messenger: Option<VulkanDebugMessengerFunctions>,

    /// The functions used by surfaces
    surface: Option<VulkanSurfaceFunctions>,

    /// The functions used by Win32 surfaces
    #[cfg(target_os = "windows")]
    win32_surface: Option<VulkanWin32SurfaceFunctions>,

    /** Specific Functions **/

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
