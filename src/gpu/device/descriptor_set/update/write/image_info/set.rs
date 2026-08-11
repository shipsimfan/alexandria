use crate::gpu::{VulkanDescriptorImageInfo, VulkanImageLayout, VulkanImageView, VulkanSampler};

impl<'a> VulkanDescriptorImageInfo<'a> {
    /// Set the sampler of the descriptor image info
    pub fn set_sampler(mut self, sampler: &'a VulkanSampler) -> Self {
        self.inner.sampler = sampler.handle();
        self
    }

    /// Set the image view of the descriptor image info
    pub fn set_image_view(mut self, image_view: &'a VulkanImageView) -> Self {
        self.inner.image_view = image_view.handle();
        self
    }

    /// Set the image layout of the descriptor image info
    pub fn set_image_layout(mut self, image_layout: VulkanImageLayout) -> Self {
        self.inner.image_layout = image_layout;
        self
    }
}
