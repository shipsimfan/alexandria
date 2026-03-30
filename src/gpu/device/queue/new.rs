use crate::gpu::{VulkanDevice, VulkanQueue};
use vulkan::VkQueue;

impl VulkanQueue {
    /// Create a new [`VulkanQueue`]
    pub(in crate::gpu::device) fn new(
        device: &VulkanDevice,
        queue_family: u32,
        queue: u32,
    ) -> VulkanQueue {
        let mut handle = VkQueue::null();

        unsafe {
            (device.functions().get_device_queue)(device.handle(), queue_family, queue, &mut handle)
        };

        VulkanQueue {
            handle,
            queue_family,
            device: device.clone(),
        }
    }
}
