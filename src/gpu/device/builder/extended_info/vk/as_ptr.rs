use crate::gpu::device::VkVulkanDeviceExtendedCreateInfo;
use std::ffi::c_void;
use vulkan::{
    VkPhysicalDeviceFeatures2, VkPhysicalDeviceVulkan13Features,
    ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt,
};

impl VkVulkanDeviceExtendedCreateInfo {
    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        match self {
            VkVulkanDeviceExtendedCreateInfo::Features(features) => {
                (features as *mut VkPhysicalDeviceFeatures2).cast()
            }
            VkVulkanDeviceExtendedCreateInfo::Vulkan13Features(features) => {
                (features as *mut VkPhysicalDeviceVulkan13Features).cast()
            }
            VkVulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                (features as *mut VkPhysicalDeviceExtendedDynamicStateFeaturesExt).cast()
            }
        }
    }
}
