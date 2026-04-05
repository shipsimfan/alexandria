use crate::gpu::{VulkanDevice, VulkanImageView};

impl VulkanImageView {
    /// Get the device that this image view is associated with
    pub(in crate::gpu::device::image_view) fn device(&self) -> &VulkanDevice {
        self.image.device()
    }
}
