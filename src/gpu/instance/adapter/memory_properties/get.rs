use crate::gpu::{VulkanAdapterMemoryProperties, VulkanMemoryHeap, VulkanMemoryType};

impl VulkanAdapterMemoryProperties {
    /// Get the number of memory heaps supported by the adapter
    pub fn num_memory_heaps(&self) -> usize {
        self.inner.memory_heap_count as _
    }

    /// Get the number of memory types supported by the adapter
    pub fn num_memory_types(&self) -> usize {
        self.inner.memory_type_count as _
    }

    /// Get the memory heaps supported by the adapter
    pub fn memory_heaps(&self) -> &[VulkanMemoryHeap] {
        unsafe { std::mem::transmute(&self.inner.memory_heaps[..self.num_memory_heaps()]) }
    }

    /// Get the memory types supported by the adapter
    pub fn memory_types(&self) -> &[VulkanMemoryType] {
        unsafe { std::mem::transmute(&self.inner.memory_types[..self.num_memory_types()]) }
    }

    /// Get the memory heap at the given index
    pub fn memory_heap(&self, index: usize) -> Option<&VulkanMemoryHeap> {
        if index < self.num_memory_heaps() {
            Some(unsafe { std::mem::transmute(&self.inner.memory_heaps[index]) })
        } else {
            None
        }
    }

    /// Get the memory type at the given index
    pub fn memory_type(&self, index: usize) -> Option<&VulkanMemoryType> {
        if index < self.num_memory_types() {
            Some(unsafe { std::mem::transmute(&self.inner.memory_types[index]) })
        } else {
            None
        }
    }
}
