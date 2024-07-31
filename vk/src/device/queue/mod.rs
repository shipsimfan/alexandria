use vulkan::VkQueue;

mod create_info;

pub use create_info::QueueCreateInfo;

/// A Vulkan queue which can be used for a variety of purposes
pub struct Queue {
    /// The handle to the queue
    handle: VkQueue,
}

impl Queue {
    /// Creates a new [`Queue`]
    pub(super) fn new(handle: VkQueue) -> Self {
        Queue { handle }
    }
}

unsafe impl Send for Queue {}
