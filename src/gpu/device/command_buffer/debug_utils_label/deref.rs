use crate::gpu::{VulkanCommandBuffer, VulkanDebugUtilsLabel};
use std::ops::{Deref, DerefMut};

impl<'a> Deref for VulkanDebugUtilsLabel<'a> {
    type Target = VulkanCommandBuffer;

    fn deref(&self) -> &Self::Target {
        &self.command_buffer
    }
}

impl<'a> DerefMut for VulkanDebugUtilsLabel<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command_buffer
    }
}
