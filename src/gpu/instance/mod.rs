use inner::VulkanInstanceInner;

mod adapter;
mod builder;
mod extension;
mod functions;
mod inner;

mod enumerate_adapters;

pub use adapter::*;
pub use builder::VulkanInstanceBuilder;
pub use extension::VulkanInstanceExtension;

pub(in crate::gpu) use functions::VulkanInstanceFunctions;

use crate::define_handle;

define_handle!(
    /// The context for interacting with Vulkan
    pub VulkanInstance -> VulkanInstanceInner
);
