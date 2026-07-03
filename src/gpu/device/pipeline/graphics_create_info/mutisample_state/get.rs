use crate::gpu::{VulkanPipelineMultisampleStateCreateInfo, VulkanSampleCountFlag};
use vulkan::VK_TRUE;

impl<'a> VulkanPipelineMultisampleStateCreateInfo<'a> {
    /// Get the number of samples used for rasterization
    pub fn rasterization_samples(&self) -> VulkanSampleCountFlag {
        self.inner.rasterization_samples
    }

    /// Get if sample shading is enabled
    pub fn sample_shading_enable(&self) -> bool {
        self.inner.sample_shading_enable == VK_TRUE
    }

    /// Get the minimum fraction of sample shading if sample shading is enabled
    pub fn min_sample_shading(&self) -> f32 {
        self.inner.min_sample_shading
    }

    /// Get if alpha to coverage is enabled
    pub fn alpha_to_coverage_enable(&self) -> bool {
        self.inner.alpha_to_coverage_enable == VK_TRUE
    }

    /// Get if alpha to one is enabled
    pub fn alpha_to_one_enable(&self) -> bool {
        self.inner.alpha_to_one_enable == VK_TRUE
    }
}
