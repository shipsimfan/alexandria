use crate::gpu::{VulkanBuffer, VulkanDescriptorBufferInfo};

impl<'a> VulkanDescriptorBufferInfo<'a> {
    /// Set the buffer of the descriptor buffer info
    pub fn set_buffer(mut self, buffer: &'a VulkanBuffer) -> Self {
        self.inner.buffer = buffer.handle();
        self
    }

    /// Set the offset of the descriptor buffer info
    pub fn set_offset(mut self, offset: u64) -> Self {
        self.inner.offset = offset;
        self
    }

    /// Set the range of the descriptor buffer info
    pub fn set_range(mut self, range: u64) -> Self {
        self.inner.range = range;
        self
    }
}
