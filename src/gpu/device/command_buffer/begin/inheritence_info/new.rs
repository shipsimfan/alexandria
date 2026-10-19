use crate::gpu::{
    VulkanCommandBufferInheritanceInfo, VulkanFramebuffer, VulkanQueryControlFlags,
    VulkanQueryPipelineStatisticFlags, VulkanRenderPass,
};
use std::marker::PhantomData;
use vulkan::{VK_FALSE, VK_TRUE, VkCommandBufferInheritanceInfo, VkFramebuffer, VkRenderPass};

impl<'a> VulkanCommandBufferInheritanceInfo<'a> {
    /// Create a new [`VulkanCommandBufferInheritanceInfo`]
    pub fn new<Qf: Into<VulkanQueryControlFlags>, Ps: Into<VulkanQueryPipelineStatisticFlags>>(
        render_pass: Option<&VulkanRenderPass>,
        subpass: u32,
        framebuffer: Option<&VulkanFramebuffer>,
        occlusion_query_enable: bool,
        query_flags: Qf,
        pipeline_statistics: Ps,
    ) -> VulkanCommandBufferInheritanceInfo<'a> {
        VulkanCommandBufferInheritanceInfo {
            inner: VkCommandBufferInheritanceInfo {
                render_pass: render_pass
                    .map_or(VkRenderPass::null(), |render_pass| render_pass.handle()),
                subpass,
                framebuffer: framebuffer
                    .map_or(VkFramebuffer::null(), |framebuffer| framebuffer.handle()),
                occlusion_query_enable: if occlusion_query_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                query_flags: query_flags.into(),
                pipeline_statistics: pipeline_statistics.into(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
