use crate::gpu::VulkanDevice;
use vulkan::VkCommandBuffer;

mod functions;

mod begin;
mod cmd_begin_rendering;
mod cmd_end_rendering;
mod cmd_pipeline_barrier2;
mod end;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

pub use cmd_begin_rendering::*;
pub use cmd_pipeline_barrier2::*;

/// A buffer for recording commands to be submitted to a Vulkan queue
pub struct VulkanCommandBuffer {
    /// The handle to the underlying Vulkan command buffer
    handle: VkCommandBuffer,

    /// The device this queue is on
    device: VulkanDevice,
}
