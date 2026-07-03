use crate::gpu::{VulkanPushConstantRange, VulkanShaderStageFlags};
use vulkan::VkPushConstantRange;

impl VulkanPushConstantRange {
    /// Create a new [`VulkanPushConstantRange`]
    pub fn new<F: Into<VulkanShaderStageFlags>>(
        stage_flags: F,
        offset: u32,
        size: u32,
    ) -> VulkanPushConstantRange {
        VulkanPushConstantRange {
            inner: VkPushConstantRange {
                stage_flags: stage_flags.into(),
                offset,
                size,
            },
        }
    }
}
