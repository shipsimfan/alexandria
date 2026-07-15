use crate::gpu::{VulkanAdapter, VulkanInstance};
use vulkan::VkPhysicalDevice;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the graphics instance this adapter came from
    pub(crate) fn instance(&self) -> &'instance VulkanInstance {
        self.instance
    }

    /// Get access to the physical device handle
    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }
}
