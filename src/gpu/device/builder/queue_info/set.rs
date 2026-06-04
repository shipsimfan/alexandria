use crate::gpu::{VulkanQueueCreateFlags, VulkanQueueCreateInfo};
use std::ops::BitOrAssign;

impl<'a> VulkanQueueCreateInfo<'a> {
    /// Set the flags to create these queues with
    pub fn set_flags<F: Into<VulkanQueueCreateFlags>>(mut self, flags: F) -> Self {
        self.inner.flags = flags.into();
        self
    }

    /// Add flags to create these queues with
    pub fn add_flags<F>(mut self, flags: F) -> Self
    where
        VulkanQueueCreateFlags: BitOrAssign<F>,
    {
        self.inner.flags |= flags.into();
        self
    }

    /// Set the index of the queue family to create these queues with
    pub fn set_queue_family(mut self, queue_family: u32) -> Self {
        self.inner.queue_family_index = queue_family;
        self
    }

    /// Set the priorities of the queues to create with
    pub fn set_priorities(mut self, priorities: &'a [f32]) -> Self {
        assert!(priorities.len() > 0);
        self.inner.queue_count = priorities.len() as _;
        self.inner.queue_priorities = priorities.as_ptr();
        self
    }
}
