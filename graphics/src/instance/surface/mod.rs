use crate::instance::GraphicsInstanceInner;
use std::sync::Arc;
use vulkan::khr_surface::VkSurfaceKhr;

mod functions;

mod drop;
mod get;
mod new;

#[cfg(target_os = "windows")]
pub(in crate::instance) use functions::Win32WindowSurfaceFunctions;
pub(in crate::instance) use functions::WindowSurfaceFunctions;

/// The surface of a window for rendering
pub struct WindowSurface {
    /// The handle to the window surface
    handle: VkSurfaceKhr,

    /// The instance this debug messenger was created for
    instance: Arc<GraphicsInstanceInner>,
}
