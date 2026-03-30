use crate::gpu::VulkanQueueCreateInfo;
use vulkan::VkDeviceQueueCreateInfo;

impl<'a> VulkanQueueCreateInfo<'a> {
    /// Convert this queue creation info into its Vulkan counter-part
    pub(in crate::gpu::device) fn to_vk(&self) -> VkDeviceQueueCreateInfo {
        VkDeviceQueueCreateInfo {
            queue_family_index: self.queue_family,
            queue_count: self.priorities.len() as _,
            queue_priorities: self.priorities.as_ptr(),
            ..Default::default()
        }
    }
}
