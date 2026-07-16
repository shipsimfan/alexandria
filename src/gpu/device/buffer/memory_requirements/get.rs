use crate::{MemorySize, gpu::VulkanMemoryRequirements};

impl VulkanMemoryRequirements {
    /// Get the size of the required memory
    pub fn size(&self) -> MemorySize {
        MemorySize::new(self.inner.memory_requirements.size)
    }

    /// Get the alignment of the required memory
    pub fn alignment(&self) -> u64 {
        self.inner.memory_requirements.alignment
    }

    /// Get the memory type bits of the required memory
    pub fn memory_type_bits(&self) -> u32 {
        self.inner.memory_requirements.memory_type_bits
    }

    /// Is `memory_type` supported by the buffer?
    pub fn is_supported(&self, memory_type: usize) -> bool {
        if memory_type >= 32 {
            return false;
        }

        (self.inner.memory_requirements.memory_type_bits & (1 << memory_type)) != 0
    }
}
