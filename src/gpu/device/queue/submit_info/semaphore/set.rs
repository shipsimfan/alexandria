use crate::gpu::{VulkanPipelineStageFlags, VulkanSemaphore, VulkanSemaphoreSubmitInfo};
use std::ops::BitOrAssign;

impl<'a> VulkanSemaphoreSubmitInfo<'a> {
    /// Set the semaphore being submitted
    pub fn set_semaphore(mut self, semaphore: &'a VulkanSemaphore) -> Self {
        self.inner.semaphore = semaphore.handle();
        self
    }

    /// Set the value used to either wait on or signal the semaphore
    pub fn set_value(mut self, value: u64) -> Self {
        self.inner.value = value;
        self
    }

    /// Set the stage mask used to wait on the semaphore
    pub fn set_stage_mask<F: Into<VulkanPipelineStageFlags>>(mut self, stage_mask: F) -> Self {
        self.inner.stage_mask = stage_mask.into();
        self
    }

    /// Add a stage to the stage mask used to wait on the semaphore
    pub fn add_stage_mask<F>(mut self, stage_mask: F) -> Self
    where
        VulkanPipelineStageFlags: BitOrAssign<F>,
    {
        self.inner.stage_mask |= stage_mask;
        self
    }

    /// Set the device index for this semaphore submit info
    pub fn set_device_index(mut self, device_index: u32) -> Self {
        self.inner.device_index = device_index;
        self
    }
}
