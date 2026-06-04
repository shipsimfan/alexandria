use crate::gpu::{
    VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanClearValue, VulkanImageLayout,
    VulkanImageView, VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
};
use vulkan::VkImageView;

impl<'a> VulkanRenderingAttachmentInfo<'a> {
    /// Set the image view for the rendering attachment
    pub fn set_image_view(mut self, image_view: &'a VulkanImageView) -> Self {
        self.inner.image_view = image_view.handle();
        self
    }

    /// Set the image layout for the rendering attachment
    pub fn set_image_layout(mut self, layout: VulkanImageLayout) -> Self {
        self.inner.image_layout = layout;
        self
    }

    /// Set the resolve mode for the rendering attachment
    pub fn set_resolve_mode(mut self, resolve_mode: VulkanResolveModeFlag) -> Self {
        self.inner.resolve_mode = resolve_mode;
        self
    }

    /// Set the resolve image view for the rendering attachment
    pub fn set_resolve_image_view(
        mut self,
        resolve_image_view: Option<&'a VulkanImageView>,
    ) -> Self {
        self.inner.resolve_image_view =
            resolve_image_view.map_or(VkImageView::null(), |view| view.handle());
        self
    }

    /// Set the resolve image layout for the rendering attachment
    pub fn set_resolve_image_layout(mut self, resolve_image_layout: VulkanImageLayout) -> Self {
        self.inner.resolve_image_layout = resolve_image_layout;
        self
    }

    /// Set the load operation for the rendering attachment
    pub fn set_load_op(mut self, load_op: VulkanAttachmentLoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }

    /// Set the store operation for the rendering attachment
    pub fn set_store_op(mut self, store_op: VulkanAttachmentStoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }

    /// Set the clear value for the rendering attachment
    pub fn set_clear_value<C: Into<VulkanClearValue>>(mut self, clear_value: C) -> Self {
        self.inner.clear_value = clear_value.into().to_vk();
        self
    }
}
