use crate::{
    Result,
    gpu::{VulkanImage, VulkanImageView, VulkanFormat},
};

impl VulkanImage {
    /// Create a new image view for this image
    pub fn create_image_view(&self, format: VulkanFormat) -> Result<VulkanImageView> {
        VulkanImageView::new(self, format)
    }
}
