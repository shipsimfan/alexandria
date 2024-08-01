use util::flags_contains;
use vk::{QueueCreateInfo, Surface, VkQueueFlagBits};

/// The indices of the queue families a device will use
pub(crate) struct QueueFamilies {
    /// The index of the graphics queue family
    graphics: u32,

    /// The index of the presenting queue family
    present: u32,
}

impl QueueFamilies {
    /// Gets the indices of the queue families for `handle`
    pub(super) fn new(physical_device: &vk::PhysicalDevice, surface: &Surface) -> Option<Self> {
        let mut graphics = None;
        let mut present = None;

        for (i, queue_family) in physical_device.queue_families().into_iter().enumerate() {
            if graphics.is_none()
                && flags_contains!(queue_family.queue_flags, VkQueueFlagBits::GraphicsBit)
            {
                graphics = Some(i as u32);
            }

            if present.is_none() && physical_device.is_surface_supported(i, surface) {
                present = Some(i as u32);
            }
        }

        Some(QueueFamilies {
            graphics: graphics?,
            present: present?,
        })
    }

    /// Gets the queue families as a slice
    pub(crate) fn to_vec(&self) -> Vec<QueueCreateInfo<'static>> {
        let mut queue_create_info = vec![QueueCreateInfo::new(self.graphics, &[1.0])];

        if self.graphics != self.present {
            queue_create_info.push(QueueCreateInfo::new(self.present, &[1.0]));
        }

        queue_create_info
    }
}
