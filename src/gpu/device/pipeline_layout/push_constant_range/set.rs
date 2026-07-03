use crate::gpu::{VulkanPushConstantRange, VulkanShaderStageFlags};

impl VulkanPushConstantRange {
    /// Set the stage flags of the push constant range
    pub fn set_stage_flags<F: Into<VulkanShaderStageFlags>>(mut self, stage_flags: F) -> Self {
        self.inner.stage_flags = stage_flags.into();
        self
    }

    /// Set the offset of the push constant range
    pub fn set_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    /// Set the size of the push constant range
    pub fn set_size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
}
