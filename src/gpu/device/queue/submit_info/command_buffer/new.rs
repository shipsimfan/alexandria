use crate::gpu::{VulkanCommandBuffer, VulkanCommandBufferSubmitInfo};
use std::marker::PhantomData;
use vulkan::VkCommandBufferSubmitInfo;

impl<'a> VulkanCommandBufferSubmitInfo<'a> {
    /// Create a new [`VulkanCommandBufferSubmitInfo`]
    pub fn new(
        command_buffer: &'a VulkanCommandBuffer,
        device_mask: u32,
    ) -> VulkanCommandBufferSubmitInfo<'a> {
        VulkanCommandBufferSubmitInfo {
            inner: VkCommandBufferSubmitInfo {
                command_buffer: command_buffer.handle(),
                device_mask,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
