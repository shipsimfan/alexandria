use crate::gpu::VulkanDeviceFeatures;
use std::ffi::c_void;
use vulkan::VkPhysicalDeviceFeatures2;

impl VulkanDeviceFeatures {
    /// Get a mutable pointer to this extended adapter info
    pub(in crate::gpu::instance::adapter::extended_info) fn as_mut_ptr(&mut self) -> *mut c_void {
        ((&mut self.inner) as *mut VkPhysicalDeviceFeatures2).cast()
    }
}
