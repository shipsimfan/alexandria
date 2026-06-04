use crate::{
    Result,
    gpu::{VulkanDevice, VulkanDeviceBuilder, VulkanQueue, device::VulkanDeviceInner},
};
use std::sync::Arc;

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`VulkanDevice`] with the provided settings
    pub fn create(&mut self) -> Result<(VulkanDevice, Vec<VulkanQueue>)> {
        let device = VulkanDevice {
            inner: Arc::new(VulkanDeviceInner::new(
                &mut self.features,
                &self.queues,
                &self.extensions,
                self.adapter,
            )?),
        };

        let mut queues = Vec::new();
        for queue_family in self.queues.iter() {
            for queue in 0..queue_family.queue_count() {
                queues.push(VulkanQueue::new(
                    &device,
                    queue_family.queue_family(),
                    queue,
                )?);
            }
        }

        Ok((device, queues))
    }
}
