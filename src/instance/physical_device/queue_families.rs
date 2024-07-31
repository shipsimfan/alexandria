use util::flags_contains;
use vk::{QueueCreateInfo, VkQueueFlagBits};

/// The indices of the queue families a device will use
pub(crate) struct QueueFamilies {
    /// The index of the graphics queue family
    graphics: u32,
}

const QUEUE_PRIORITY: f32 = 1.0;

impl QueueFamilies {
    /// Gets the indices of the queue families for `handle`
    pub(super) fn new(physical_device: &vk::PhysicalDevice) -> Option<Self> {
        let mut graphics = None;

        for (i, queue_family) in physical_device.queue_families().into_iter().enumerate() {
            if graphics.is_none()
                && flags_contains!(queue_family.queue_flags, VkQueueFlagBits::GraphicsBit)
            {
                graphics = Some(i as u32);
            }
        }

        Some(QueueFamilies {
            graphics: graphics?,
        })
    }

    /// Gets the index of the graphics queue family
    pub(crate) fn graphics(&self) -> u32 {
        self.graphics
    }

    /// Gets the queue families as a slice
    pub(crate) fn to_vec(&self) -> Vec<QueueCreateInfo<'static>> {
        vec![QueueCreateInfo::new(self.graphics, &[1.0])]
    }
}
