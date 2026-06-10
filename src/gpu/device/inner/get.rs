use crate::gpu::{
    VulkanInstance,
    device::{VulkanDeviceFunctions, VulkanDeviceInner},
};
use vulkan::{VkDevice, VkPhysicalDevice};

impl VulkanDeviceInner {
    /// Get accesss to the device handle
    pub fn handle(&self) -> VkDevice {
        self.handle
    }

    /// Get the device level functions
    pub fn functions(&self) -> &VulkanDeviceFunctions {
        &self.functions
    }

    /// Get access to the instance that the device was created from
    pub fn instance(&self) -> &VulkanInstance {
        &self.instance
    }

    /// Get the physical device that the device was created from
    pub fn physical_device(&self) -> VkPhysicalDevice {
        self.physical_device
    }
}
