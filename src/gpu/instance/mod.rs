use inner::VulkanInstanceInner;

mod adapter;
mod builder;
mod debug_messenger;
mod extension;
mod functions;
mod inner;

mod create_debug_messenger;
mod enumerate_adapters;

pub use adapter::*;
pub use builder::VulkanInstanceBuilder;
pub use debug_messenger::*;
pub use extension::VulkanInstanceExtension;

pub(in crate::gpu) use functions::VulkanInstanceFunctions;

use crate::define_handle;

define_handle!(
    /// The context for interacting with Vulkan
    pub VulkanInstance -> VulkanInstanceInner
);
