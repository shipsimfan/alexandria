use crate::define_handle;

mod builder;
mod extension;
mod image;
mod image_view;
mod inner;
mod queue;
mod swapchain;

mod create_swapchain;
mod get;

pub use builder::*;
pub use extension::*;
pub use image::*;
pub use image_view::*;
pub use queue::*;
pub use swapchain::*;

pub(in crate::gpu::device) use inner::*;

define_handle!(
    /// An interface for interacting with a specific Vulkan device
    pub VulkanDevice -> VulkanDeviceInner
);
