use crate::gpu::{VulkanMemoryPropertyFlag, VulkanMemoryPropertyFlags, VulkanMemoryType};

impl VulkanMemoryType {
    /// Get the flags of the memory type
    pub fn flags(&self) -> VulkanMemoryPropertyFlags {
        self.inner.property_flags
    }

    /// Get the index of the memory heap that this memory type belongs to
    pub fn heap_index(&self) -> usize {
        self.inner.heap_index as _
    }

    /// Is this memory type device local?
    pub fn device_local(&self) -> bool {
        self.flags().contains(VulkanMemoryPropertyFlag::DeviceLocal)
    }

    /// Is this memory type host visible?
    pub fn host_visible(&self) -> bool {
        self.flags().contains(VulkanMemoryPropertyFlag::HostVisible)
    }

    /// Is this memory type host coherent?
    pub fn host_coherent(&self) -> bool {
        self.flags()
            .contains(VulkanMemoryPropertyFlag::HostCoherent)
    }
}
