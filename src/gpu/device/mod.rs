use crate::define_handle;

mod buffer;
mod builder;
mod command_buffer;
mod command_pool;
mod extension;
mod fence;
mod image;
mod image_view;
mod inner;
mod queue;
mod semaphore;
mod swapchain;

mod create_command_pool;
mod create_fence;
mod create_semaphore;
mod create_swapchain;
mod get;
mod surface_capabilities;
mod wait_idle;

pub use buffer::*;
pub use builder::*;
pub use command_buffer::*;
pub use command_pool::*;
pub use extension::*;
pub use fence::*;
pub use image::*;
pub use image_view::*;
pub use queue::*;
pub use semaphore::*;
pub use swapchain::*;

pub(in crate::gpu::device) use inner::*;

define_handle!(
    /// An interface for interacting with a specific Vulkan device
    pub VulkanDevice -> VulkanDeviceInner
);
