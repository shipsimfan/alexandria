use crate::gpu::VulkanSemaphoreType;
use vulkan::{VkSemaphoreType, VkSemaphoreTypeCreateInfo};

impl VulkanSemaphoreType {
    /// Converts the VulkanSemaphoreType to its corresponding Vulkan representation
    pub(in crate::gpu::device::semaphore) fn to_vk(&self) -> Option<VkSemaphoreTypeCreateInfo> {
        match self {
            VulkanSemaphoreType::Binary => None,
            &VulkanSemaphoreType::Timeline(initial_value) => Some(VkSemaphoreTypeCreateInfo {
                semaphore_type: VkSemaphoreType::Timeline,
                initial_value,
                ..Default::default()
            }),
        }
    }
}
