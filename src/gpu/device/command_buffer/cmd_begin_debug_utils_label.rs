use crate::gpu::{VulkanCommandBuffer, VulkanDebugUtilsLabel};
use dioptra::{Color4f, Srgb};
use std::ffi::CStr;

impl VulkanCommandBuffer {
    /// Begins a debug utils label region in the command buffer
    pub fn begin_debug_utils_label<'a, C: Into<Color4f<Srgb>>>(
        &'a mut self,
        name: &CStr,
        color: C,
    ) -> VulkanDebugUtilsLabel<'a> {
        VulkanDebugUtilsLabel::new(name, color.into(), self)
    }
}
