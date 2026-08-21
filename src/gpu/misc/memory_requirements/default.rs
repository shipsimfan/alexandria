use crate::gpu::VulkanMemoryRequirements;
use vulkan::{VkMemoryRequirements, VkMemoryRequirements2};

impl Default for VulkanMemoryRequirements {
    fn default() -> Self {
        VulkanMemoryRequirements {
            inner: VkMemoryRequirements2 {
                memory_requirements: VkMemoryRequirements {
                    size: 0,
                    alignment: 1,
                    memory_type_bits: u32::MAX,
                },
                ..Default::default()
            },
        }
    }
}
