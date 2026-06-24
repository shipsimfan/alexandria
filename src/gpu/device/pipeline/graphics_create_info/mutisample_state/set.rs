use crate::gpu::{VulkanPipelineMultisampleStateCreateInfo, VulkanSampleCountFlag};
use std::ptr::null;
use vulkan::{VK_FALSE, VK_TRUE};

impl<'a> VulkanPipelineMultisampleStateCreateInfo<'a> {
    /// Set the number of samples used for rasterization
    pub fn set_rasterization_samples(
        mut self,
        rasterization_samples: VulkanSampleCountFlag,
    ) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }

    /// Enable sample shading
    pub fn enable_sample_shading(mut self) -> Self {
        self.inner.sample_shading_enable = VK_TRUE;
        self
    }

    /// Disable sample shading
    pub fn disable_sample_shading(mut self) -> Self {
        self.inner.sample_shading_enable = VK_FALSE;
        self
    }

    /// Set the minimum fraction of sample shading if sample shading is enabled
    pub fn set_min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.min_sample_shading = min_sample_shading;
        self
    }

    /// Set the sample mask if sample shading is enabled
    pub fn set_sample_mask(mut self, sample_mask: Option<&'a [u32]>) -> Self {
        self.inner.sample_mask = sample_mask.map_or(null(), |mask| mask.as_ptr());
        self
    }

    /// Enable alpha to coverage
    pub fn enable_alpha_to_coverage(mut self) -> Self {
        self.inner.alpha_to_coverage_enable = VK_TRUE;
        self
    }

    /// Disable alpha to coverage
    pub fn disable_alpha_to_coverage(mut self) -> Self {
        self.inner.alpha_to_coverage_enable = VK_FALSE;
        self
    }

    /// Enable alpha to one
    pub fn enable_alpha_to_one(mut self) -> Self {
        self.inner.alpha_to_one_enable = VK_TRUE;
        self
    }
}
