use alexandria::{
    Id,
    gpu::{
        VulkanColorSpace, VulkanCommandBuffer, VulkanCommandPool, VulkanDevice, VulkanFormat,
        VulkanPresentMode, VulkanQueue,
    },
};
use debug_messenger::DebugMessenger;
use frame_data::FrameData;

mod debug_messenger;
mod frame_data;
mod swapchain;

mod new;
mod wait_idle;

pub use swapchain::Swapchain;

const SWAPCHAIN_FORMAT: VulkanFormat = VulkanFormat::B8G8R8A8Srgb;
const SWAPCHAIN_COLOR_SPACE: VulkanColorSpace = VulkanColorSpace::SRGBNonlinearKhr;
const SWAPCHAIN_PRESENT_MODE: VulkanPresentMode = VulkanPresentMode::FIFOKhr;
const MAX_FRAMES_IN_FLIGHT: usize = 2;

/// The context for rendering
pub struct RenderContext {
    /// The debug messenger for this context, if any
    _debug_messenger: DebugMessenger,

    /// The Vulkan device to render with
    device: VulkanDevice,

    /// The queue to submit rendering commands to
    queue: VulkanQueue,

    /// The command pool to allocate command buffers from
    command_pool: VulkanCommandPool,

    /// The allocated command buffers for each frame in flight
    command_buffers: Vec<Id<VulkanCommandBuffer>>,
}
