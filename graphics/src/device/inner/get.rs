use crate::device::inner::GraphicsDeviceInner;
use vulkan::VkDevice;

impl GraphicsDeviceInner {
    /// Get accesss to the device handle
    pub(in crate::device) fn handle(&self) -> VkDevice {
        self.handle
    }
}
