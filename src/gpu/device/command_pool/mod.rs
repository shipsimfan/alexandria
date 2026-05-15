use crate::define_handle;
use inner::VulkanCommandPoolInner;

mod functions;
mod inner;

mod allocate_command_buffer;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

define_handle!(
    /// A pool of command buffers for a Vulkan device
    pub VulkanCommandPool -> VulkanCommandPoolInner
);
