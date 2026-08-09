use crate::gpu::VulkanCopyDescriptorSet;

impl<'a> VulkanCopyDescriptorSet<'a> {
    /// Get the index of the source binding in the source descriptor set
    pub fn src_binding(&self) -> u32 {
        self.inner.src_binding
    }

    /// Get the array element index in the source descriptor set
    pub fn src_array_element(&self) -> u32 {
        self.inner.src_array_element
    }

    /// Get the index of the destination binding in the destination descriptor set
    pub fn dst_binding(&self) -> u32 {
        self.inner.dst_binding
    }

    /// Get the array element index in the destination descriptor set
    pub fn dst_array_element(&self) -> u32 {
        self.inner.dst_array_element
    }

    /// Get the number of descriptors to copy from the source to the destination
    pub fn descriptor_count(&self) -> u32 {
        self.inner.descriptor_count
    }
}
