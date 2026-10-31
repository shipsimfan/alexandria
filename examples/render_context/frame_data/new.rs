use crate::render_context::FrameData;
use alexandria::gpu::{VulkanDevice, VulkanFenceCreateFlag, VulkanSemaphoreType};

impl FrameData {
    /// Creates a new [`FrameData`]
    pub fn new(device: &VulkanDevice) -> Self {
        let render_complete_semaphore = device
            .create_semaphore(VulkanSemaphoreType::Binary)
            .unwrap();
        let acquire_image_semaphore = device
            .create_semaphore(VulkanSemaphoreType::Binary)
            .unwrap();
        let draw_fence = device
            .create_fence(VulkanFenceCreateFlag::Signalled)
            .unwrap();

        FrameData {
            render_complete_semaphore,
            acquire_image_semaphore,
            draw_fence,
        }
    }
}
