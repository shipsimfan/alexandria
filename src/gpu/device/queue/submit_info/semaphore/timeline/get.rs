use crate::gpu::VulkanTimelineSemaphoreSubmitInfo;

impl<'a> VulkanTimelineSemaphoreSubmitInfo<'a> {
    /// Get the wait semaphore values
    pub fn wait_values(&self) -> &'a [u64] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.wait_semaphore_values,
                self.inner.wait_semaphore_value_count as usize,
            )
        }
    }

    /// Get the signal semaphore values
    pub fn signal_values(&self) -> &'a [u64] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.signal_semaphore_values,
                self.inner.signal_semaphore_value_count as usize,
            )
        }
    }
}
