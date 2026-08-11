use crate::gpu::{VulkanDescriptorImageInfo, VulkanImageLayout};

impl<'a> VulkanDescriptorImageInfo<'a> {
    /// Get the image layout of the descriptor image info
    pub fn image_layout(&self) -> VulkanImageLayout {
        self.inner.image_layout
    }
}
