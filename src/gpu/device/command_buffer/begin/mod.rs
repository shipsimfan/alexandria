use crate::{Error, Result, gpu::VulkanCommandBuffer};
use std::ptr::null;
use vulkan::{VkCommandBufferBeginInfo, try_vulkan};

mod inheritence_info;

pub use inheritence_info::*;

impl VulkanCommandBuffer {
    /// Start recording commands into this command buffer
    pub fn begin(
        &mut self,
        inheritence_info: Option<VulkanCommandBufferInheritanceInfo>,
    ) -> Result<()> {
        let begin_info = VkCommandBufferBeginInfo {
            inheritance_info: inheritence_info.as_ref().map_or(null(), |d| {
                (d as *const VulkanCommandBufferInheritanceInfo).cast()
            }),
            ..Default::default()
        };

        try_vulkan!((self
            .device
            .functions()
            .command_buffer
            .begin_command_buffer)(self.handle, &begin_info))
        .map_err(|vk| Error::new_with("unable to begin command buffer", vk))?;

        Ok(())
    }
}
