use crate::gpu::{VulkanQueueCreateFlags, VulkanQueueCreateInfo};

impl<'a> VulkanQueueCreateInfo<'a> {
    /// Get the flags to create these queues with
    pub fn flags(&self) -> VulkanQueueCreateFlags {
        self.inner.flags
    }

    /// Get the index of the queue family to create these queues with
    pub fn queue_family(&self) -> u32 {
        self.inner.queue_family_index
    }

    /// Get the number of queues to create
    pub fn queue_count(&self) -> u32 {
        self.inner.queue_count
    }

    /// Get the priorities of the queues to create
    pub fn priorities(&self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self.inner.queue_priorities, self.inner.queue_count as usize)
        }
    }
}
