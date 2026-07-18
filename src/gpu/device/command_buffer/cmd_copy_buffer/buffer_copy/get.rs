use crate::gpu::VulkanBufferCopy;

impl VulkanBufferCopy {
    /// Get the source offset for the buffer copy region
    pub fn src_offset(&self) -> u64 {
        self.inner.src_offset
    }

    /// Get the destination offset for the buffer copy region
    pub fn dst_offset(&self) -> u64 {
        self.inner.dst_offset
    }

    /// Get the size of the buffer copy region
    pub fn size(&self) -> u64 {
        self.inner.size
    }
}
