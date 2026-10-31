use crate::gpu::{
    VulkanPipelineStageFlags, VulkanSemaphore, VulkanSemaphoreSubmitInfo,
    VulkanTimelineSemaphoreSubmitInfo,
};
use std::{marker::PhantomData, ptr::null};
use vulkan::VkSemaphoreSubmitInfo;

impl<'a> VulkanSemaphoreSubmitInfo<'a> {
    /// Create a new [`VulkanSemaphoreSubmitInfo`]
    pub fn new<F: Into<VulkanPipelineStageFlags>>(
        semaphore: &'a VulkanSemaphore,
        value: u64,
        stage_mask: F,
        device_index: u32,
        timeline: Option<&'a VulkanTimelineSemaphoreSubmitInfo>,
    ) -> VulkanSemaphoreSubmitInfo<'a> {
        VulkanSemaphoreSubmitInfo {
            inner: VkSemaphoreSubmitInfo {
                semaphore: semaphore.handle(),
                value,
                stage_mask: stage_mask.into(),
                device_index,
                next: timeline.map_or(null(), |timeline| {
                    (timeline as *const VulkanTimelineSemaphoreSubmitInfo).cast()
                }),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
