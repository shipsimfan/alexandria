use crate::gpu::{VulkanPushConstantRange, VulkanShaderStageFlags};

impl VulkanPushConstantRange {
    /// Get the stage flags of the push constant range
    pub fn stage_flags(&self) -> VulkanShaderStageFlags {
        self.inner.stage_flags
    }

    /// Get the offset of the push constant range
    pub fn offset(&self) -> u32 {
        self.inner.offset
    }

    /// Get the size of the push constant range
    pub fn size(&self) -> u32 {
        self.inner.size
    }
}
