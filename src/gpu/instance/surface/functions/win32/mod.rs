use crate::FunctionSymbol;
use vulkan::khr_win32_surface::VkCreateWin32SurfaceKhr;

mod load;

/// Functions used by Win32 surfaces
pub(in crate::gpu::instance) struct VulkanWin32SurfaceFunctions {
    /// The function to create a surface for Win32
    pub create_win32_surface: FunctionSymbol<VkCreateWin32SurfaceKhr>,
}
