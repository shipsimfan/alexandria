use crate::gpu::VulkanDescriptorBufferInfo;

impl<'a> VulkanDescriptorBufferInfo<'a> {
    /// Get the offset of the descriptor buffer info
    pub fn offset(&self) -> u64 {
        self.inner.offset
    }

    /// Get the range of the descriptor buffer info
    pub fn range(&self) -> u64 {
        self.inner.range
    }
}
