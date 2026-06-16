use crate::gpu::VulkanSpecializationMapEntry;
use vulkan::VkSpecializationMapEntry;

impl VulkanSpecializationMapEntry {
    /// Create a new [`VulkanSpecializationMapEntry`]
    pub fn new(constant_id: u32, offset: u32, size: usize) -> VulkanSpecializationMapEntry {
        VulkanSpecializationMapEntry {
            inner: VkSpecializationMapEntry {
                constant_id,
                offset,
                size,
            },
        }
    }
}
