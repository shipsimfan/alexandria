use crate::define_handle;

mod buffer;
mod builder;
mod command_buffer;
mod command_pool;
mod descriptor_set_layout;
mod extension;
mod fence;
mod image;
mod image_view;
mod inner;
mod pipeline;
mod pipeline_cache;
mod pipeline_layout;
mod queue;
mod render_pass;
mod semaphore;
mod shader_module;
mod swapchain;

mod create_buffer;
mod create_command_pool;
mod create_fence;
mod create_graphics_pipeline;
mod create_pipeline_layout;
mod create_semaphore;
mod create_shader_module;
mod create_swapchain;
mod get;
mod surface_capabilities;
mod wait_idle;

pub use buffer::*;
pub use builder::*;
pub use command_buffer::*;
pub use command_pool::*;
pub use descriptor_set_layout::*;
pub use extension::*;
pub use fence::*;
pub use image::*;
pub use image_view::*;
pub use pipeline::*;
pub use pipeline_cache::*;
pub use pipeline_layout::*;
pub use queue::*;
pub use render_pass::*;
pub use semaphore::*;
pub use shader_module::*;
pub use swapchain::*;

pub(in crate::gpu::device) use inner::*;

define_handle!(
    /// An interface for interacting with a specific Vulkan device
    pub VulkanDevice -> VulkanDeviceInner
);
