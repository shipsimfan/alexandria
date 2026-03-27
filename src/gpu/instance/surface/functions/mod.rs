use crate::FunctionSymbol;
use vulkan::khr_surface::VkDestroySurfaceKhr;

mod win32;

mod load;

pub(in crate::gpu::instance) use win32::*;

/// Functions used by surfaces
pub(in crate::gpu::instance) struct VulkanSurfaceFunctions {
    /// The function to destroy a surface
    pub destroy_surface: FunctionSymbol<VkDestroySurfaceKhr>,
}
