use crate::gpu::VulkanInstance;
use vulkan::khr_surface::VkSurfaceKhr;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::instance) use functions::*;

/// A platform-native surface or window that can be rendered to
pub struct VulkanSurface {
    /// The handle to the underlying Vulkan surface
    handle: VkSurfaceKhr,

    /// The Vulkan instance that this surface was created from
    instance: VulkanInstance,
}
