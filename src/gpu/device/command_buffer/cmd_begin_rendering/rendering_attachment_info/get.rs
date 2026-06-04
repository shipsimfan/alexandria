use crate::gpu::{
    VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanImageLayout,
    VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
};

impl<'a> VulkanRenderingAttachmentInfo<'a> {
    /// Get the image layout of the rendering attachment
    pub fn image_layout(&self) -> VulkanImageLayout {
        self.inner.image_layout
    }

    /// Get the resolve mode of the rendering attachment
    pub fn resolve_mode(&self) -> VulkanResolveModeFlag {
        self.inner.resolve_mode
    }

    /// Get the resolve image layout of the rendering attachment
    pub fn resolve_image_layout(&self) -> VulkanImageLayout {
        self.inner.resolve_image_layout
    }

    /// Get the load operation for the rendering attachment
    pub fn load_op(&self) -> VulkanAttachmentLoadOp {
        self.inner.load_op
    }

    /// Get the store operation for the rendering attachment
    pub fn store_op(&self) -> VulkanAttachmentStoreOp {
        self.inner.store_op
    }
}
