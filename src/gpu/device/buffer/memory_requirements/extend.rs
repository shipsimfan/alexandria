use crate::gpu::VulkanMemoryRequirements;
use vulkan::{VkMemoryRequirements, VkMemoryRequirements2};

impl VulkanMemoryRequirements {
    /// Extend the memory requirements to contain another [`VulkanMemoryRequirements`]
    pub fn extend(&self, other: &VulkanMemoryRequirements) -> VulkanMemoryRequirements {
        let size = self
            .inner
            .memory_requirements
            .size
            .next_multiple_of(other.inner.memory_requirements.alignment);
        let alignment = self
            .inner
            .memory_requirements
            .alignment
            .max(other.inner.memory_requirements.alignment);
        let memory_type_bits = self.inner.memory_requirements.memory_type_bits
            & other.inner.memory_requirements.memory_type_bits;

        VulkanMemoryRequirements {
            inner: VkMemoryRequirements2 {
                memory_requirements: VkMemoryRequirements {
                    size,
                    alignment,
                    memory_type_bits,
                },
                ..Default::default()
            },
        }
    }
}
