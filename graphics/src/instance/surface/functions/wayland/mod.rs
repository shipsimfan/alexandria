use vulkan::khr_wayland_surface::VkCreateWaylandSurfaceKhr;

mod load;

/// Functions used by Wayland window surfaces
pub(in crate::instance) struct WaylandWindowSurfaceFunctions {
    /// The function to create a Win32 surface
    pub create_surface: VkCreateWaylandSurfaceKhr,
}
