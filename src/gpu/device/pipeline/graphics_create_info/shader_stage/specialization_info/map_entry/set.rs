use crate::gpu::VulkanSpecializationMapEntry;

impl VulkanSpecializationMapEntry {
    /// Set the `constant_id` of the specialization map entry
    pub fn set_constant_id(mut self, constant_id: u32) -> Self {
        self.inner.constant_id = constant_id;
        self
    }

    /// Set the `offset` of the specialization map entry
    pub fn set_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    /// Set the `size` of the specialization map entry
    pub fn set_size(mut self, size: usize) -> Self {
        self.inner.size = size;
        self
    }
}
