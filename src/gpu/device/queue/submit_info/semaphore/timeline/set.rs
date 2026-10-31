use crate::gpu::VulkanTimelineSemaphoreSubmitInfo;

impl<'a> VulkanTimelineSemaphoreSubmitInfo<'a> {
    /// Set the wait semaphore values
    pub fn set_wait_values(&mut self, wait_values: &'a [u64]) {
        self.inner.wait_semaphore_values = wait_values.as_ptr();
        self.inner.wait_semaphore_value_count = wait_values.len() as u32;
    }

    /// Set the signal semaphore values
    pub fn set_signal_values(&mut self, signal_values: &'a [u64]) {
        self.inner.signal_semaphore_values = signal_values.as_ptr();
        self.inner.signal_semaphore_value_count = signal_values.len() as u32;
    }
}
