use crate::render_context::{
    RenderContext, SWAPCHAIN_FORMAT, SWAPCHAIN_PRESENT_MODE, Swapchain, frame_data::FrameData,
};
use alexandria::{gpu::VulkanSurface, window::Window};

impl<'surface> Swapchain<'surface> {
    /// Create a new swapchain for the given surface and window
    pub fn new(
        render_context: &RenderContext,
        surface: &'surface VulkanSurface,
        window: &Window<()>,
    ) -> Self {
        let swapchain = render_context
            .device
            .create_swapchain(
                3,
                SWAPCHAIN_FORMAT,
                window.size(),
                SWAPCHAIN_PRESENT_MODE,
                &surface,
            )
            .unwrap();
        let mut image_views = Vec::with_capacity(swapchain.images().len());
        for image in swapchain.images() {
            image_views.push(image.create_image_view(SWAPCHAIN_FORMAT).unwrap());
        }

        // Create per-frame data
        let frame_data = [
            FrameData::new(&render_context.device, &render_context.command_pool),
            FrameData::new(&render_context.device, &render_context.command_pool),
        ];

        Swapchain {
            swapchain,
            size: window.size(),
            image_views,
            frame_data,
            frame_index: 0,
        }
    }
}
