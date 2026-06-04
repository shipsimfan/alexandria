use crate::gpu::{
    VulkanCommandBufferSubmitInfo, VulkanSemaphoreSubmitInfo, VulkanSubmitFlags, VulkanSubmitInfo,
};
use std::marker::PhantomData;
use vulkan::VkSubmitInfo2;

impl<'a> VulkanSubmitInfo<'a> {
    /// Creates a new [`VulkanSubmitInfo`]
    pub fn new<F: Into<VulkanSubmitFlags>>(
        flags: F,
        wait_semaphores: &'a [VulkanSemaphoreSubmitInfo<'a>],
        command_buffers: &'a [VulkanCommandBufferSubmitInfo<'a>],
        signal_semaphores: &'a [VulkanSemaphoreSubmitInfo<'a>],
    ) -> VulkanSubmitInfo<'a> {
        VulkanSubmitInfo {
            inner: VkSubmitInfo2 {
                flags: flags.into(),
                wait_semaphore_info_count: wait_semaphores.len() as u32,
                wait_semaphore_infos: wait_semaphores.as_ptr().cast(),
                command_buffer_info_count: command_buffers.len() as u32,
                command_buffer_infos: command_buffers.as_ptr().cast(),
                signal_semaphore_info_count: signal_semaphores.len() as u32,
                signal_semaphore_infos: signal_semaphores.as_ptr().cast(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
