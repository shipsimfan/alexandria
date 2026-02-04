use crate::{
    Error, Result,
    gpu::{GpuSubsystem, VulkanVersion},
};
use vulkan::try_vulkan;

impl GpuSubsystem {
    /// Get the maximum supported Vulkan version of the current system
    pub fn version(&self) -> Result<VulkanVersion> {
        // Get the version
        let mut version = 0;
        try_vulkan!((self.functions.enumerate_instance_version)(&mut version))
            .map_err(|vk| Error::new_with("unable to get Vulkan version", vk))?;

        Ok(unsafe { VulkanVersion::new_raw(version) })
    }
}
