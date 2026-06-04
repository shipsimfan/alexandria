use crate::gpu::{VulkanPipelineStageFlags, VulkanSemaphore, VulkanSemaphoreSubmitInfo};
use std::marker::PhantomData;
use vulkan::VkSemaphoreSubmitInfo;

impl<'a> VulkanSemaphoreSubmitInfo<'a> {
    /// Create a new [`VulkanSemaphoreSubmitInfo`]
    pub fn new<F: Into<VulkanPipelineStageFlags>>(
        semaphore: &'a VulkanSemaphore,
        value: u64,
        stage_mask: F,
        device_index: u32,
    ) -> VulkanSemaphoreSubmitInfo<'a> {
        VulkanSemaphoreSubmitInfo {
            inner: VkSemaphoreSubmitInfo {
                semaphore: semaphore.handle(),
                value,
                stage_mask: stage_mask.into(),
                device_index,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
