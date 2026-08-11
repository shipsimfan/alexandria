use crate::gpu::{VulkanDescriptorType, VulkanWriteDescriptorSet};

impl<'a> VulkanWriteDescriptorSet<'a> {
    /// Get the destination binding index for the write operation
    pub fn dst_binding(&self) -> u32 {
        self.inner.dst_binding
    }

    /// Get the destination array element index for the write operation
    pub fn dst_array_element(&self) -> u32 {
        self.inner.dst_array_element
    }

    /// Get the number of descriptors to write in the write operation
    pub fn descriptor_count(&self) -> usize {
        self.inner.descriptor_count as _
    }

    /// Get the descriptor type for the write operation
    pub fn descriptor_type(&self) -> VulkanDescriptorType {
        self.inner.descriptor_type
    }
}
