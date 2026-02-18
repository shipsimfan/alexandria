use crate::{
    Result,
    gpu::{GpuSubsystem, VulkanLayer},
};

impl GpuSubsystem {
    /// Enumerate all Vulkan layers supported on this system
    pub fn layers(&self) -> Result<Vec<VulkanLayer>> {
        self.inner.layers()
    }
}
