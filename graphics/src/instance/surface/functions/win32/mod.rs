use vulkan::khr_win32_surface::VkCreateWin32SurfaceKhr;

mod load;

/// Functions used by Win32 window surfaces
pub(in crate::instance) struct Win32WindowSurfaceFunctions {
    /// The function to create a Win32 surface
    pub create_surface: VkCreateWin32SurfaceKhr,
}
