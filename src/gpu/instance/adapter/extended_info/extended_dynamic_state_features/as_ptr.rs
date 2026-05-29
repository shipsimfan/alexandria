use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;
use std::ffi::c_void;
use vulkan::ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt;

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Get a mutable pointer to this extended adapter info
    pub(in crate::gpu::instance::adapter::extended_info) fn as_mut_ptr(&mut self) -> *mut c_void {
        ((&mut self.inner) as *mut VkPhysicalDeviceExtendedDynamicStateFeaturesExt).cast()
    }
}
