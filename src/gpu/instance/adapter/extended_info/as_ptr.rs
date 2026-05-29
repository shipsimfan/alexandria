use crate::gpu::VulkanExtendedAdapterInfo;
use std::ffi::c_void;

impl VulkanExtendedAdapterInfo {
    /// Get a mutable pointer to this extended adapter info
    pub(in crate::gpu) fn as_mut_ptr(&mut self) -> *mut c_void {
        match self {
            VulkanExtendedAdapterInfo::Features(features) => features.as_mut_ptr(),
            VulkanExtendedAdapterInfo::Vulkan13Features(features) => features.as_mut_ptr(),
            VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features) => {
                features.as_mut_ptr()
            }
        }
    }
}
