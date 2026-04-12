use inner::VulkanCommandPoolInner;
use std::sync::{Arc, Mutex};

mod inner;

/// A pool of command buffers for a Vulkan device
#[derive(Clone)]
pub struct VulkanCommandPool {
    /// The inner command pool handle
    inner: Arc<Mutex<VulkanCommandPoolInner>>,
}
