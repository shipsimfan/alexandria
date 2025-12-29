use crate::{GraphicsError, GraphicsVersion, Result};
use std::ptr::null_mut;
use vulkan::{
    VK_ENUMERATE_INSTANCE_VERSION, VkEnumerateInstanceVersion, try_vulkan, vkGetInstanceProcAddr,
};

impl GraphicsVersion {
    /// Get the maximum supported Vulkan version of the current system
    pub fn enumerate() -> Result<GraphicsVersion> {
        // Get the "vkEnumerateInstanceVersion" function
        let enumerate_instance_version: VkEnumerateInstanceVersion = unsafe {
            std::mem::transmute(
                vkGetInstanceProcAddr(null_mut(), VK_ENUMERATE_INSTANCE_VERSION.as_ptr()).ok_or(
                    GraphicsError::new("unable to find \"vkEnumerateInstanceVersion\""),
                )?,
            )
        };

        // Get the version
        let mut version = 0;
        try_vulkan!(enumerate_instance_version(&mut version))
            .map_err(|vk| GraphicsError::new_vk("unable to enumerate instance version", vk))?;

        Ok(unsafe { GraphicsVersion::new_raw(version) })
    }
}
