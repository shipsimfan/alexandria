use crate::{
    Result,
    gpu::{GpuSubsystem, VulkanVersion},
};

impl GpuSubsystem {
    /// Get the maximum supported Vulkan version of the current system
    pub fn version(&self) -> Result<VulkanVersion> {
        self.inner.version()
    }
}
