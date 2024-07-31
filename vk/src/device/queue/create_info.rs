use std::borrow::Cow;
use vulkan::VkDeviceQueueCreateInfo;

/// A description of queues to create in a family
pub struct QueueCreateInfo<'a> {
    /// The queue family to create from
    queue_family: u32,

    /// The priorities of the created queues
    priorities: Cow<'a, [f32]>,
}

impl<'a> QueueCreateInfo<'a> {
    /// Creates a new [`QueueCreateInfo`]
    pub fn new<P: Into<Cow<'a, [f32]>>>(queue_family: u32, priorities: P) -> Self {
        QueueCreateInfo {
            queue_family,
            priorities: priorities.into(),
        }
    }

    /// Gets the queue family to create the queues in the
    pub(crate) fn to_vk(&self) -> VkDeviceQueueCreateInfo {
        VkDeviceQueueCreateInfo {
            queue_family_index: self.queue_family,
            queue_count: self.priorities.len() as _,
            queue_priorities: self.priorities.as_ptr(),
            ..Default::default()
        }
    }
}
