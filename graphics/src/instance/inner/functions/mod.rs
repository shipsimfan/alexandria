#[cfg(target_os = "linux")]
use crate::instance::WaylandWindowSurfaceFunctions;
#[cfg(target_os = "windows")]
use crate::instance::Win32WindowSurfaceFunctions;
use crate::instance::{
    GraphicsAdapterFunctions, GraphicsDebugMessengerFunctions, WindowSurfaceFunctions,
};
use vulkan::{VkDestroyInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr};

mod get;
mod load;

/// The functions loaded for a specific graphics instance
pub(crate) struct GraphicsInstanceFunctions {
    /// The functions used by adapters
    pub(in crate::instance) adapter: GraphicsAdapterFunctions,

    /// The functions used by debug messenger
    debug_messenger: Option<GraphicsDebugMessengerFunctions>,

    /// The functions used by surfaces
    surface: Option<WindowSurfaceFunctions>,

    /// The functions used by Win32 surfaces
    #[cfg(target_os = "windows")]
    win32_surface: Option<Win32WindowSurfaceFunctions>,

    /// The functions used by Wayland surfaces
    #[cfg(target_os = "linux")]
    wayland_surface: Option<WaylandWindowSurfaceFunctions>,

    /// The function to get an address of device procedure
    pub get_device_proc_addr: VkGetDeviceProcAddr,

    /// The function used to enumerate the physical devices on the system
    pub(in crate::instance) enumerate_physical_devices: VkEnumeratePhysicalDevices,

    /// The function used to destroy the instance
    pub(in crate::instance) destroy_instance: VkDestroyInstance,
}
