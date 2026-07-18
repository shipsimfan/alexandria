use crate::{Error, Result, gpu::VulkanQueue};
use vulkan::try_vulkan;

impl VulkanQueue {
    /// Waits for the queue to be idle, blocking until all pending operations have completed
    pub fn wait_idle(&self) -> Result<()> {
        try_vulkan!((self.device.functions().queue.queue_wait_idle)(self.handle))
            .map_err(|vk| Error::new_with("unable to wait for queue to idle", vk))?;

        Ok(())
    }
}
