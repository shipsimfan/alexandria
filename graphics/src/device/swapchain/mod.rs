use crate::{GpuImage, WindowSurface, device::GraphicsDeviceInner};
use std::sync::Arc;
use vulkan::khr_swapchain::VkSwapchainKhr;

mod format;
mod functions;
mod present_mode;

mod drop;
mod get;
mod new;

pub use format::SwapchainFormat;
pub use present_mode::SwapchainPresentMode;

pub(in crate::device) use functions::SwapchainFunctions;

/// A swapchain used for presenting rendered images on window surfaces
pub struct Swapchain<'surface> {
    /// The handle to the underlying swapchain
    handle: VkSwapchainKhr,

    /// The set of images that make up this swapchain
    images: Vec<GpuImage>,

    /// The surface this swapchain was created for
    _surface: &'surface WindowSurface,

    /// The device that created this swapchain
    device: Arc<GraphicsDeviceInner>,
}
