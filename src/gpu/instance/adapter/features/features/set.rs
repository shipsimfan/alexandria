use crate::gpu::VulkanDeviceFeatures;
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanDeviceFeatures {
    /// Enable support for sampler anisotropy
    pub fn enable_sampler_anisotropy(mut self) -> Self {
        self.inner.features.sampler_anisotropy = VK_TRUE;
        self
    }

    /// Disable support for sampler anisotropy
    pub fn disable_sampler_anisotropy(mut self) -> Self {
        self.inner.features.sampler_anisotropy = VK_FALSE;
        self
    }
}
