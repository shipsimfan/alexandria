use crate::gpu::VulkanTimelineSemaphoreSubmitInfo;
use std::marker::PhantomData;
use vulkan::VkTimelineSemaphoreSubmitInfo;

impl<'a> VulkanTimelineSemaphoreSubmitInfo<'a> {
    /// Create a new [`VulkanTimelineSemaphoreSubmitInfo`]
    pub fn new(
        wait_values: &'a [u64],
        signal_values: &'a [u64],
    ) -> VulkanTimelineSemaphoreSubmitInfo<'a> {
        VulkanTimelineSemaphoreSubmitInfo {
            inner: VkTimelineSemaphoreSubmitInfo {
                wait_semaphore_values: wait_values.as_ptr(),
                wait_semaphore_value_count: wait_values.len() as u32,
                signal_semaphore_values: signal_values.as_ptr(),
                signal_semaphore_value_count: signal_values.len() as u32,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
