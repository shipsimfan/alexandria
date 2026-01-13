use crate::GraphicsQueueFamilyInfo;
use vulkan::{VkQueueFamilyProperties, VkQueueFlag};

impl GraphicsQueueFamilyInfo {
    /// Create a new [`GraphicsQueueFamilyInfo`] from info, unless it is implicitly not supported
    pub(in crate::instance::adapter) fn new(
        index: u32,
        info: VkQueueFamilyProperties,
    ) -> Option<GraphicsQueueFamilyInfo> {
        if info.queue_count == 0 {
            return None;
        }

        let graphics = info.queue_flags.contains(VkQueueFlag::GraphicsBit);
        let compute = info.queue_flags.contains(VkQueueFlag::ComputeBit);
        let transfer = info.queue_flags.contains(VkQueueFlag::TransferBit);

        if !graphics && !compute && !transfer {
            return None;
        }

        Some(GraphicsQueueFamilyInfo {
            index,
            count: info.queue_count,
            graphics,
            compute,
            transfer,
        })
    }
}
