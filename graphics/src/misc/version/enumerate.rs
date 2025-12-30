use crate::{GraphicsError, GraphicsVersion, Result, util::load_global_function};
use vulkan::{VK_ENUMERATE_INSTANCE_VERSION, VkEnumerateInstanceVersion, try_vulkan};

impl GraphicsVersion {
    /// Get the maximum supported Vulkan version of the current system
    pub fn enumerate() -> Result<GraphicsVersion> {
        // Get the "vkEnumerateInstanceVersion" function
        let enumerate_instance_version: VkEnumerateInstanceVersion =
            load_global_function!(VK_ENUMERATE_INSTANCE_VERSION)?;

        // Get the version
        let mut version = 0;
        try_vulkan!(enumerate_instance_version(&mut version))
            .map_err(|vk| GraphicsError::new_vk("unable to enumerate instance version", vk))?;

        Ok(unsafe { GraphicsVersion::new_raw(version) })
    }
}
