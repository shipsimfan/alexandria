use crate::gpu::VulkanQueueCreateInfo;

impl<'a> VulkanQueueCreateInfo<'a> {
    /// Create a new [`VulkanQueueCreateInfo`]
    pub fn new(queue_family: u32, priorities: &'a [f32]) -> VulkanQueueCreateInfo<'a> {
        assert!(priorities.len() > 0);

        VulkanQueueCreateInfo {
            queue_family,
            priorities,
        }
    }
}
