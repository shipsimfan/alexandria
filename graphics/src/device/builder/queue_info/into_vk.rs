use crate::GraphicsQueueCreateInfo;
use vulkan::VkDeviceQueueCreateInfo;

impl<'a> GraphicsQueueCreateInfo<'a> {
    /// Convert this queue creation info into its Vulkan counter-part
    pub(in crate::device) fn into_vk(&self) -> VkDeviceQueueCreateInfo {
        VkDeviceQueueCreateInfo {
            queue_family_index: self.queue_family,
            queue_count: self.priorities.len() as _,
            queue_priorities: self.priorities.as_ptr(),
            ..Default::default()
        }
    }
}
