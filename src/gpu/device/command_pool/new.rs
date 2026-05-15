use crate::{
    Result,
    gpu::{VulkanCommandPool, VulkanDevice, device::command_pool::VulkanCommandPoolInner},
};

impl VulkanCommandPool {
    /// Create a new [`VulkanCommandPool`]
    pub(in crate::gpu::device) fn new(
        queue_family: u32,
        device: VulkanDevice,
    ) -> Result<VulkanCommandPool> {
        VulkanCommandPoolInner::new(queue_family, device).map(VulkanCommandPool::from_inner)
    }
}
