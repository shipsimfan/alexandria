use crate::gpu::VulkanBufferCopy;

impl VulkanBufferCopy {
    /// Set the source offset for the buffer copy region
    pub fn set_src_offset(&mut self, src_offset: u64) {
        self.inner.src_offset = src_offset;
    }

    /// Set the destination offset for the buffer copy region
    pub fn set_dst_offset(&mut self, dst_offset: u64) {
        self.inner.dst_offset = dst_offset;
    }

    /// Set the size of the buffer copy region
    pub fn set_size(&mut self, size: u64) {
        self.inner.size = size;
    }
}
