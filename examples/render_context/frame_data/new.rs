use crate::render_context::FrameData;
use alexandria::gpu::{VulkanDevice, VulkanFenceCreateFlag};

impl FrameData {
    /// Creates a new [`FrameData`]
    pub fn new(device: &VulkanDevice) -> Self {
        let render_complete_semaphore = device.create_semaphore().unwrap();
        let acquire_image_semaphore = device.create_semaphore().unwrap();
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
