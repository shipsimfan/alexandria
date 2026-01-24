use crate::{GraphicsDevice, GraphicsQueue};
use vulkan::VkQueue;

impl GraphicsQueue {
    /// Create a new [`GraphicsQueue`]
    pub(in crate::device) fn new(
        device: &GraphicsDevice,
        queue_family: u32,
        queue: u32,
    ) -> GraphicsQueue {
        let mut handle = VkQueue::null();

        (device.functions.get_device_queue)(device.handle(), queue_family, queue, &mut handle);

        GraphicsQueue {
            handle,
            queue_family,
            device: device.clone(),
        }
    }
}
