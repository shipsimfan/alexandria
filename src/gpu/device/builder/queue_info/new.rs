use crate::gpu::VulkanQueueCreateInfo;
use std::marker::PhantomData;
use vulkan::VkDeviceQueueCreateInfo;

impl<'a> VulkanQueueCreateInfo<'a> {
    /// Create a new [`VulkanQueueCreateInfo`]
    pub fn new(queue_family: u32, priorities: &'a [f32]) -> VulkanQueueCreateInfo<'a> {
        assert!(priorities.len() > 0);

        VulkanQueueCreateInfo {
            inner: VkDeviceQueueCreateInfo {
                queue_family_index: queue_family,
                queue_count: priorities.len() as _,
                queue_priorities: priorities.as_ptr(),
                ..Default::default()
            },
            _priorities: PhantomData,
        }
    }
}
