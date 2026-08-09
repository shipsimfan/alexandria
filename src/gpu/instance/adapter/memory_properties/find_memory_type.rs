use crate::gpu::{VulkanAdapterMemoryProperties, VulkanMemoryPropertyFlags};

impl VulkanAdapterMemoryProperties {
    /// Finds a suitable memory type index based on the provided type filter and desired memory
    /// properties
    pub fn find_memory_type<F: Into<VulkanMemoryPropertyFlags>>(
        &self,
        type_filter: u32,
        properties: F,
    ) -> Option<usize> {
        let properties = properties.into();

        for (i, memory_type) in self.inner.memory_types.iter().enumerate() {
            if (type_filter & (1 << i)) != 0 && memory_type.property_flags.contains(properties) {
                return Some(i);
            }
        }

        None
    }
}
