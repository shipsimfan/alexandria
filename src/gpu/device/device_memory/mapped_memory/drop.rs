use crate::gpu::VulkanMappedMemory;

impl<T> Drop for VulkanMappedMemory<T> {
    fn drop(&mut self) {
        self.do_unmap();
    }
}
