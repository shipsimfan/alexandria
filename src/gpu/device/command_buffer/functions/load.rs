use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanCommandBufferFunctions, load_device_function},
};
use vulkan::{
    VK_ALLOCATE_COMMAND_BUFFERS, VK_BEGIN_COMMAND_BUFFER, VK_CMD_BEGIN_RENDERING,
    VK_CMD_BIND_INDEX_BUFFER, VK_CMD_BIND_PIPELINE, VK_CMD_BIND_VERTEX_BUFFERS, VK_CMD_COPY_BUFFER,
    VK_CMD_DRAW, VK_CMD_DRAW_INDEXED, VK_CMD_END_RENDERING, VK_CMD_PIPELINE_BARRIER2,
    VK_CMD_PUSH_CONSTANTS, VK_CMD_SET_SCISSOR, VK_CMD_SET_VIEWPORT, VK_END_COMMAND_BUFFER,
    VK_FREE_COMMAND_BUFFERS, VkDevice,
};

impl VulkanCommandBufferFunctions {
    /// Load all the required command buffer functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanCommandBufferFunctions> {
        Ok(VulkanCommandBufferFunctions {
            allocate_command_buffers: load_device_function!(
                instance,
                device,
                VK_ALLOCATE_COMMAND_BUFFERS
            )?,
            free_command_buffers: load_device_function!(instance, device, VK_FREE_COMMAND_BUFFERS)?,
            begin_command_buffer: load_device_function!(instance, device, VK_BEGIN_COMMAND_BUFFER)?,
            end_command_buffer: load_device_function!(instance, device, VK_END_COMMAND_BUFFER)?,

            cmd_pipeline_barrier2: load_device_function!(
                instance,
                device,
                VK_CMD_PIPELINE_BARRIER2
            )?,
            cmd_begin_rendering: load_device_function!(instance, device, VK_CMD_BEGIN_RENDERING)?,
            cmd_end_rendering: load_device_function!(instance, device, VK_CMD_END_RENDERING)?,
            cmd_bind_pipeline: load_device_function!(instance, device, VK_CMD_BIND_PIPELINE)?,
            cmd_set_viewport: load_device_function!(instance, device, VK_CMD_SET_VIEWPORT)?,
            cmd_set_scissor: load_device_function!(instance, device, VK_CMD_SET_SCISSOR)?,
            cmd_draw: load_device_function!(instance, device, VK_CMD_DRAW)?,
            cmd_bind_vertex_buffers: load_device_function!(
                instance,
                device,
                VK_CMD_BIND_VERTEX_BUFFERS
            )?,
            cmd_copy_buffer: load_device_function!(instance, device, VK_CMD_COPY_BUFFER)?,
            cmd_bind_index_buffer: load_device_function!(
                instance,
                device,
                VK_CMD_BIND_INDEX_BUFFER
            )?,
            cmd_draw_indexed: load_device_function!(instance, device, VK_CMD_DRAW_INDEXED)?,
            cmd_push_constants: load_device_function!(instance, device, VK_CMD_PUSH_CONSTANTS)?,
        })
    }
}
