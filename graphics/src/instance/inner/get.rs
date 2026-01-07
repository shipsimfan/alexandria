use crate::instance::inner::GraphicsInstanceInner;
use vulkan::VkInstance;

impl GraphicsInstanceInner {
    /// Get accesss to the instance handle
    pub(in crate::instance) fn handle(&self) -> VkInstance {
        self.handle
    }
}
