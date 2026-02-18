use crate::{
    Result,
    gpu::{GpuSubsystem, VulkanExtension, VulkanLayer},
};

impl GpuSubsystem {
    /// Enumerate all Vulkan extensions supported on this system, even if Alexandria doesn't
    /// support them
    pub fn all_extensions(&self, layer: Option<&VulkanLayer>) -> Result<Vec<VulkanExtension>> {
        self.inner.all_extensions(layer)
    }
}
