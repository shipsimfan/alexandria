use crate::gpu::{
    VulkanAccessFlags, VulkanCommandBuffer, VulkanImage, VulkanImageLayout,
    VulkanPipelineStageFlags,
};
use vulkan::{
    VK_QUEUE_FAMILY_IGNORED, VkDependencyInfo, VkImageAspectFlag, VkImageMemoryBarrier2,
    VkImageSubresourceRange,
};

impl VulkanCommandBuffer {
    /// Insert a pipeline barrier into the command buffer to transition an image between layouts
    pub fn cmd_pipeline_barrier2<
        A1: Into<VulkanAccessFlags>,
        A2: Into<VulkanAccessFlags>,
        P1: Into<VulkanPipelineStageFlags>,
        P2: Into<VulkanPipelineStageFlags>,
    >(
        &mut self,
        image: &VulkanImage,
        old_layout: VulkanImageLayout,
        new_layout: VulkanImageLayout,
        src_access_mask: A1,
        dst_access_mask: A2,
        src_stage_mask: P1,
        dst_stage_mask: P2,
    ) {
        let barrier = VkImageMemoryBarrier2 {
            src_stage_mask: src_stage_mask.into(),
            src_access_mask: src_access_mask.into(),
            dst_stage_mask: dst_stage_mask.into(),
            dst_access_mask: dst_access_mask.into(),
            old_layout,
            new_layout,
            src_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
            image: image.handle(),
            subresource_range: VkImageSubresourceRange {
                aspect_mask: VkImageAspectFlag::Color.into(),
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let dependency_info = VkDependencyInfo {
            image_memory_barrier_count: 1,
            image_memory_barriers: &barrier,
            ..Default::default()
        };

        unsafe {
            (self
                .command_pool
                .device()
                .functions()
                .command_buffer
                .cmd_pipeline_barrier2)(self.handle, &dependency_info)
        };
    }
}
