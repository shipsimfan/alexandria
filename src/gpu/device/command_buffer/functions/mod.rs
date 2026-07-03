use crate::FunctionSymbol;
use vulkan::{
    VkAllocateCommandBuffers, VkBeginCommandBuffer, VkCmdBeginRendering, VkCmdBindPipeline,
    VkCmdDraw, VkCmdEndRendering, VkCmdPipelineBarrier2, VkCmdSetScissor, VkCmdSetViewport,
    VkEndCommandBuffer, VkFreeCommandBuffers,
};

mod load;

/// The functions that are used by command buffers associated with a device
pub(in crate::gpu::device) struct VulkanCommandBufferFunctions {
    /// The function to allocate command buffers
    pub allocate_command_buffers: FunctionSymbol<VkAllocateCommandBuffers>,

    /// The function to free command buffers
    pub free_command_buffers: FunctionSymbol<VkFreeCommandBuffers>,

    /// Start recording commands into a command buffer
    pub begin_command_buffer: FunctionSymbol<VkBeginCommandBuffer>,

    /// End recording commands into a command buffer
    pub end_command_buffer: FunctionSymbol<VkEndCommandBuffer>,

    /// The function to insert a pipeline barrier into a command buffer
    pub cmd_pipeline_barrier2: FunctionSymbol<VkCmdPipelineBarrier2>,

    /// The function to begin a dynamic rendering pass
    pub cmd_begin_rendering: FunctionSymbol<VkCmdBeginRendering>,

    /// The function to end a dynamic rendering pass
    pub cmd_end_rendering: FunctionSymbol<VkCmdEndRendering>,

    /// The function to bind a graphics or compute pipeline to a command buffer
    pub cmd_bind_pipeline: FunctionSymbol<VkCmdBindPipeline>,

    /// The function to set the viewport for subsequent draw calls in a command buffer
    pub cmd_set_viewport: FunctionSymbol<VkCmdSetViewport>,

    /// The function to set the scissor rectangle for subsequent draw calls in a command buffer
    pub cmd_set_scissor: FunctionSymbol<VkCmdSetScissor>,

    /// The function to draw some vertices in a command buffer
    pub cmd_draw: FunctionSymbol<VkCmdDraw>,
}
