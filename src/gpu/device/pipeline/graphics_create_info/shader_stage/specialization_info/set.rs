use crate::gpu::{VulkanSpecializationInfo, VulkanSpecializationMapEntry};

impl<'a> VulkanSpecializationInfo<'a> {
    /// Set the map entries of the specialization info
    pub fn set_map_entries(mut self, map_entries: &'a [VulkanSpecializationMapEntry]) -> Self {
        self.inner.map_entry_count = map_entries.len() as _;
        self.inner.map_entries = map_entries.as_ptr().cast();
        self
    }

    /// Set the data of the specialization info
    pub fn set_data(mut self, data: &'a [u8]) -> Self {
        self.inner.data_size = data.len();
        self.inner.data = data.as_ptr().cast();
        self
    }
}
