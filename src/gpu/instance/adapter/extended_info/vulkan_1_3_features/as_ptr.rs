use crate::gpu::VulkanDeviceVulkan13Features;
use std::ffi::c_void;
use vulkan::VkPhysicalDeviceVulkan13Features;

impl VulkanDeviceVulkan13Features {
    /// Get a mutable pointer to this extended adapter info
    pub(in crate::gpu::instance::adapter::extended_info) fn as_mut_ptr(&mut self) -> *mut c_void {
        ((&mut self.inner) as *mut VkPhysicalDeviceVulkan13Features).cast()
    }
}
