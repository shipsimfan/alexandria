use crate::{
    Result,
    gpu::{VulkanCommandPool, VulkanDevice, device::command_pool::VulkanCommandPoolInner},
};
use std::sync::{Arc, Mutex};

impl VulkanCommandPool {
    /// Create a new [`VulkanCommandPool`]
    pub(in crate::gpu::device) fn new(
        queue_family: u32,
        device: VulkanDevice,
    ) -> Result<VulkanCommandPool> {
        Ok(VulkanCommandPool {
            inner: Arc::new(Mutex::new(VulkanCommandPoolInner::new(
                queue_family,
                device,
            )?)),
        })
    }
}
