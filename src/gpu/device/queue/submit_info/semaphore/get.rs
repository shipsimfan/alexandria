use crate::gpu::{
    VulkanPipelineStageFlags, VulkanSemaphoreSubmitInfo, VulkanTimelineSemaphoreSubmitInfo,
};

impl<'a> VulkanSemaphoreSubmitInfo<'a> {
    /// Get the value used to either wait on or signal the semaphore
    pub fn value(&self) -> u64 {
        self.inner.value
    }

    /// Get the stage mask used to wait on the semaphore
    pub fn stage_mask(&self) -> VulkanPipelineStageFlags {
        self.inner.stage_mask
    }

    /// Get the device index for this semaphore submit info
    pub fn device_index(&self) -> u32 {
        self.inner.device_index
    }

    /// Get the timeline semaphore info
    pub fn timeline_info(&self) -> Option<&'a VulkanTimelineSemaphoreSubmitInfo<'a>> {
        unsafe { (self.inner.next as *const VulkanTimelineSemaphoreSubmitInfo<'a>).as_ref() }
    }
}
