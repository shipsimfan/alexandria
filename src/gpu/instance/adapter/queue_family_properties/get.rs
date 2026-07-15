use crate::{
    gpu::{VulkanQueueFamilyProperties, VulkanQueueFlag, VulkanQueueFlags},
    math::Vector3u,
};

impl VulkanQueueFamilyProperties {
    /// Get the number of queues this family contains
    pub fn count(&self) -> u32 {
        self.inner.queue_count
    }

    /// Get the flags of this queue family
    pub fn flags(&self) -> VulkanQueueFlags {
        self.inner.queue_flags
    }

    /// Does the flags of this queue family contain the given flag?
    pub fn contains_flag<F: Into<VulkanQueueFlags>>(&self, flag: F) -> bool {
        self.flags().contains(flag)
    }

    /// Does this queue support graphics commands?
    pub fn graphics(&self) -> bool {
        self.contains_flag(VulkanQueueFlag::Graphics)
    }

    /// Does this queue support compute commands?
    pub fn compute(&self) -> bool {
        self.contains_flag(VulkanQueueFlag::Compute)
    }

    /// Does this queue support transfer commands?
    pub fn transfer(&self) -> bool {
        self.contains_flag(VulkanQueueFlag::Transfer)
    }

    /// Get the valid bits of the timestamp counter for this queue family
    pub fn timestamp_valid_bits(&self) -> u32 {
        self.inner.timestamp_valid_bits
    }

    /// Get the minimum granularity for image transfer operations on this queue family
    pub fn min_image_transfer_granularity(&self) -> Vector3u {
        self.inner.min_image_transfer_granularity.into()
    }
}
