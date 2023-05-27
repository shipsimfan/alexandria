pub(crate) struct QueueFamilyIndices {
    graphics: Option<u32>,
}

impl QueueFamilyIndices {
    pub(super) fn get(physical_device: &vulkan::PhysicalDevice) -> Self {
        let queue_families = physical_device.get_queue_family_properties();

        let mut graphics = None;
        for i in 0..queue_families.len() {
            if queue_families[i]
                .queue_flags
                .contains(vulkan::QueueFlagBits::Graphics)
            {
                graphics = Some(i as u32);
                break;
            }
        }

        QueueFamilyIndices { graphics }
    }

    pub(super) fn has_graphics(&self) -> bool {
        self.graphics.is_some()
    }

    pub(crate) fn graphics(&self) -> u32 {
        self.graphics.unwrap()
    }
}
