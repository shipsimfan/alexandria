use inner::VulkanInstanceInner;

mod adapter;
mod builder;
mod debug_messenger;
mod extension;
mod functions;
mod inner;
mod surface;

mod create_debug_messenger;
mod create_window_surface;
mod enumerate_adapters;
mod get;

pub use adapter::*;
pub use builder::*;
pub use debug_messenger::*;
pub use extension::*;
pub use surface::*;

pub(in crate::gpu) use functions::VulkanInstanceFunctions;

use crate::define_handle;

define_handle!(
    /// The context for interacting with Vulkan
    pub VulkanInstance -> VulkanInstanceInner
);
