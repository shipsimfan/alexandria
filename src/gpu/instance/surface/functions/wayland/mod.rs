use crate::FunctionSymbol;
use vulkan::khr_wayland_surface::VkCreateWaylandSurfaceKhr;

mod load;

/// Functions used by Wayland surfaces
pub(in crate::gpu::instance) struct VulkanWaylandSurfaceFunctions {
    /// The function to create a surface for Wayland
    pub create_wayland_surface: FunctionSymbol<VkCreateWaylandSurfaceKhr>,
}
