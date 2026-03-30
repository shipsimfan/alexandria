use crate::{
    Result,
    gpu::{VulkanDevice, VulkanDeviceBuilder, VulkanQueue, device::VulkanDeviceInner},
};
use std::sync::Arc;

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`VulkanDevice`] with the provided settings
    pub fn create(&self) -> Result<(VulkanDevice, Vec<VulkanQueue>)> {
        let device = VulkanDevice {
            inner: Arc::new(VulkanDeviceInner::new(
                &self.extended_info,
                &self.queues,
                &self.extensions,
                self.adapter,
            )?),
        };

        let mut queues = Vec::new();
        for (queue_family_index, queue_family) in self.queues.iter().enumerate() {
            for queue in 0..queue_family.priorities.len() {
                queues.push(VulkanQueue::new(
                    &device,
                    queue_family_index as _,
                    queue as _,
                ));
            }
        }

        Ok((device, queues))
    }
}
