use crate::gpu::{
    VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanClearValue, VulkanImageLayout,
    VulkanImageView, VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
};
use std::marker::PhantomData;
use vulkan::{VkImageView, VkRenderingAttachmentInfo};

impl<'a> VulkanRenderingAttachmentInfo<'a> {
    /// Create a new [`VulkanRenderingAttachmentInfo`]
    pub fn new<C: Into<VulkanClearValue>>(
        image_view: &'a VulkanImageView,
        image_layout: VulkanImageLayout,
        resolve_mode: VulkanResolveModeFlag,
        resolve_image_view: Option<&'a VulkanImageView>,
        resolve_image_layout: VulkanImageLayout,
        load_op: VulkanAttachmentLoadOp,
        store_op: VulkanAttachmentStoreOp,
        clear_value: C,
    ) -> VulkanRenderingAttachmentInfo<'a> {
        VulkanRenderingAttachmentInfo {
            inner: VkRenderingAttachmentInfo {
                image_view: image_view.handle(),
                image_layout,
                resolve_mode,
                resolve_image_view: resolve_image_view.map_or(VkImageView::null(), |v| v.handle()),
                resolve_image_layout,
                load_op,
                store_op,
                clear_value: clear_value.into().to_vk(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
