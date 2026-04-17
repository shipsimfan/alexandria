use inner::VulkanCommandPoolInner;
use std::sync::{Arc, Mutex};

mod functions;
mod inner;

mod allocate_command_buffer;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A pool of command buffers for a Vulkan device
#[derive(Clone)]
pub struct VulkanCommandPool {
    /// The inner command pool handle
    inner: Arc<Mutex<VulkanCommandPoolInner>>,
}
