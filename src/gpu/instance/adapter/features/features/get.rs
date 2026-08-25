use crate::gpu::VulkanDeviceFeatures;
use vulkan::VK_TRUE;

impl VulkanDeviceFeatures {
    /// Is sampler anisotropy supported by the adapter?
    pub fn sampler_anisotropy(&self) -> bool {
        self.inner.features.sampler_anisotropy == VK_TRUE
    }
}
