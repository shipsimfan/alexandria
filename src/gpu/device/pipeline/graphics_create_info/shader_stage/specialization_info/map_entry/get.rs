use crate::gpu::VulkanSpecializationMapEntry;

impl VulkanSpecializationMapEntry {
    /// Get the `constant_id` of the specialization map entry
    pub fn constant_id(&self) -> u32 {
        self.inner.constant_id
    }

    /// Get the `offset` of the specialization map entry
    pub fn offset(&self) -> u32 {
        self.inner.offset
    }

    /// Get the `size` of the specialization map entry
    pub fn size(&self) -> usize {
        self.inner.size
    }
}
