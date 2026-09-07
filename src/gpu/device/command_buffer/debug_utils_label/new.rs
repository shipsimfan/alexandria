use crate::gpu::{VulkanCommandBuffer, VulkanDebugUtilsLabel};
use dioptra::{Color4f, Srgb};
use std::ffi::CStr;
use vulkan::ext_debug_utils::VkDebugUtilsLabelExt;

impl<'a> VulkanDebugUtilsLabel<'a> {
    /// Create a new [`VulkanDebugUtilsLabel`]
    pub(in crate::gpu::device::command_buffer) fn new(
        label: &CStr,
        color: Color4f<Srgb>,
        command_buffer: &'a mut VulkanCommandBuffer,
    ) -> VulkanDebugUtilsLabel<'a> {
        unsafe {
            (command_buffer
                .device
                .functions()
                .command_buffer
                .debug_utils()
                .cmd_begin_debug_utils_label)(
                command_buffer.handle,
                &VkDebugUtilsLabelExt {
                    label_name: label.as_ptr(),
                    color: color.into(),
                    ..Default::default()
                },
            )
        };

        VulkanDebugUtilsLabel { command_buffer }
    }
}
