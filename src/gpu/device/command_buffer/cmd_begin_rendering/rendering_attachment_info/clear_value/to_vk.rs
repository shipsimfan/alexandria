use crate::gpu::VulkanClearValue;
use vulkan::{VkClearColorValue, VkClearDepthStencilValue, VkClearValue};

impl VulkanClearValue {
    /// Convert this clear value into the corresponding Vulkan [`VkClearValue`]
    pub(in crate::gpu::device::command_buffer::cmd_begin_rendering::rendering_attachment_info) fn to_vk(
        &self,
    ) -> VkClearValue {
        match self {
            VulkanClearValue::ColorF32(color) => VkClearValue {
                color: VkClearColorValue { float32: *color },
            },
            VulkanClearValue::ColorI32(color) => VkClearValue {
                color: VkClearColorValue { int32: *color },
            },
            VulkanClearValue::ColorU32(color) => VkClearValue {
                color: VkClearColorValue { uint32: *color },
            },
            VulkanClearValue::DepthStencil { depth, stencil } => VkClearValue {
                depth_stencil: VkClearDepthStencilValue {
                    depth: *depth,
                    stencil: *stencil,
                },
            },
        }
    }
}
