use crate::{Error, Result, gpu::VulkanDevice};
use vulkan::try_vulkan;

impl VulkanDevice {
    /// Waits for the device to be idle, blocking until all pending operations have completed
    pub fn wait_idle(&self) -> Result<()> {
        try_vulkan!((self.functions().device_wait_idle)(self.handle()))
            .map_err(|vk| Error::new_with("unable to wait for device to idle", vk))?;

        Ok(())
    }
}
