use crate::gpu::VulkanAdapter;
use std::cmp::Ordering;
use vulkan::VkPhysicalDeviceType;

impl<'instance> PartialOrd for VulkanAdapter<'instance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'instance> Ord for VulkanAdapter<'instance> {
    fn cmp(&self, other: &Self) -> Ordering {
        match device_kind_score(self.kind).cmp(&device_kind_score(other.kind)) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match other.vram.cmp(&self.vram) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match other.api_version.cmp(&self.api_version) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match other.queue_families.len().cmp(&self.queue_families.len()) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.name.cmp(&other.name) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match other.driver_version.cmp(&self.driver_version) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        self.uuid.cmp(&other.uuid)
    }
}

/// Get a ranking for `device_kind`, with lower values being better
fn device_kind_score(device_kind: VkPhysicalDeviceType) -> u8 {
    match device_kind {
        VkPhysicalDeviceType::DiscreteGPU => 0,
        VkPhysicalDeviceType::IntegratedGPU => 1,
        VkPhysicalDeviceType::VirtualGPU => 2,
        VkPhysicalDeviceType::CPU => 3,
        VkPhysicalDeviceType::Other => 4,
        _ => 5,
    }
}
