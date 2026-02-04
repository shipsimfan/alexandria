use crate::gpu::VulkanQueueFamilyInfo;
use vulkan::{VkQueueFamilyProperties, VkQueueFlag};

impl VulkanQueueFamilyInfo {
    /// Create a new [`VulkanQueueFamilyInfo`] from info, unless it is implicitly not supported
    pub(in crate::gpu::instance::adapter) fn new(
        index: u32,
        info: VkQueueFamilyProperties,
    ) -> Option<VulkanQueueFamilyInfo> {
        if info.queue_count == 0 {
            return None;
        }

        let graphics = info.queue_flags.contains(VkQueueFlag::GraphicsBit);
        let compute = info.queue_flags.contains(VkQueueFlag::ComputeBit);
        let transfer = info.queue_flags.contains(VkQueueFlag::TransferBit);

        if !graphics && !compute && !transfer {
            return None;
        }

        Some(VulkanQueueFamilyInfo {
            index,
            count: info.queue_count,
            graphics,
            compute,
            transfer,
        })
    }
}
