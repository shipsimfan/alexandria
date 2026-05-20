use crate::render_context::FrameData;
use alexandria::gpu::{VulkanCommandPool, VulkanDevice};

impl FrameData {
    /// Creates a new [`FrameData`]
    pub fn new(device: &VulkanDevice, command_pool: &VulkanCommandPool) -> Self {
        let command_buffer = command_pool.allocate_command_buffer().unwrap();
        let render_complete_semaphore = device.create_semaphore().unwrap();
        let present_complete_semaphore = device.create_semaphore().unwrap();
        let draw_fence = device.create_fence(true).unwrap();

        FrameData {
            command_buffer,
            render_complete_semaphore,
            present_complete_semaphore,
            draw_fence,
        }
    }
}
