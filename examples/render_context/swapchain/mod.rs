use crate::render_context::FrameData;
use alexandria::{
    gpu::{VulkanImageView, VulkanSwapchain},
    math::Vector2i,
};

mod new;
mod render_frame;

/// The swapchain and image views
pub struct Swapchain<'surface> {
    /// The swapchain to render to
    swapchain: VulkanSwapchain<'surface>,

    /// The size of the swapchain
    size: Vector2i,

    /// The image views for the swapchain images
    image_views: Vec<VulkanImageView>,

    /// The frame data for the each frame in flight
    frame_data: Vec<FrameData>,

    /// The index of the next frame data element to use
    frame_index: usize,
}
