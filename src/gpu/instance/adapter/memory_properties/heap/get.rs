use crate::{
    MemorySize,
    gpu::{VulkanMemoryHeap, VulkanMemoryHeapFlags},
};

impl VulkanMemoryHeap {
    /// Get the size of the memory heap in bytes
    pub fn size(&self) -> MemorySize {
        MemorySize::new(self.inner.size)
    }

    /// Get the flags of the memory heap
    pub fn flags(&self) -> VulkanMemoryHeapFlags {
        self.inner.flags
    }
}
