use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanQueue},
};
use vulkan::{VkQueue, try_vulkan};

impl VulkanQueue {
    /// Create a new [`VulkanQueue`]
    pub(in crate::gpu::device) fn new(
        device: &VulkanDevice,
        queue_family: u32,
        queue: u32,
    ) -> Result<VulkanQueue> {
        let mut handle = VkQueue::null();
        try_vulkan!((device.functions().get_device_queue)(
            device.handle(),
            queue_family,
            queue,
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to get queue from graphics device", error))?;

        Ok(VulkanQueue {
            handle,
            queue_family,
            device: device.clone(),
        })
    }
}
