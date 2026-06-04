use crate::render_context::{
    MAX_FRAMES_IN_FLIGHT, RenderContext, SWAPCHAIN_COLOR_SPACE, SWAPCHAIN_FORMAT,
    SWAPCHAIN_PRESENT_MODE, Swapchain, frame_data::FrameData,
};
use alexandria::{
    gpu::{
        VulkanCommandBufferLevel, VulkanComponentMapping, VulkanCompositeAlphaFlag,
        VulkanImageAspectFlag, VulkanImageUsageFlag, VulkanImageViewType, VulkanSharingMode,
        VulkanSurface, VulkanSurfaceTransformFlag,
    },
    window::Window,
};

impl<'surface> Swapchain<'surface> {
    /// Create a new swapchain for the given surface and window
    pub fn new(
        render_context: &mut RenderContext,
        surface: &'surface mut VulkanSurface,
        window: &Window<()>,
    ) -> Self {
        let swapchain = render_context
            .device
            .create_swapchain(
                0,
                surface,
                MAX_FRAMES_IN_FLIGHT as _,
                SWAPCHAIN_FORMAT,
                SWAPCHAIN_COLOR_SPACE,
                window.size(),
                1,
                VulkanImageUsageFlag::ColorAttachment,
                VulkanSharingMode::Exclusive,
                &[],
                VulkanSurfaceTransformFlag::IdentityKhr,
                VulkanCompositeAlphaFlag::OpaqueKhr,
                SWAPCHAIN_PRESENT_MODE,
                true,
                None,
            )
            .unwrap();
        let mut image_views = Vec::with_capacity(swapchain.images().len());
        let mut frame_data = Vec::with_capacity(swapchain.images().len());
        for image in swapchain.images() {
            image_views.push(
                image
                    .create_image_view(
                        0,
                        VulkanImageViewType::_2d,
                        SWAPCHAIN_FORMAT,
                        VulkanComponentMapping::default(),
                        VulkanImageAspectFlag::Color,
                        0,
                        1,
                        0,
                        1,
                    )
                    .unwrap(),
            );

            frame_data.push(FrameData::new(&render_context.device));
        }

        if render_context.command_buffers.len() < swapchain.images().len() {
            for _ in render_context.command_buffers.len()..swapchain.images().len() {
                render_context.command_buffers.push(
                    render_context
                        .command_pool
                        .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
                        .unwrap(),
                );
            }
        }

        Swapchain {
            swapchain,
            size: window.size(),
            image_views,
            frame_data,
            frame_index: 0,
        }
    }
}
