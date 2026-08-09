use crate::gpu::{VulkanCopyDescriptorSet, VulkanDescriptorSet};

impl<'a> VulkanCopyDescriptorSet<'a> {
    /// Set the source descriptor set for the copy operation
    pub fn set_src_set(mut self, src_set: &'a VulkanDescriptorSet) -> Self {
        self.inner.src_set = src_set.handle;
        self
    }

    /// Set the source binding index in the source descriptor set
    pub fn set_src_binding(mut self, src_binding: u32) -> Self {
        self.inner.src_binding = src_binding;
        self
    }

    /// Set the source array element index in the source descriptor set
    pub fn set_src_array_element(mut self, src_array_element: u32) -> Self {
        self.inner.src_array_element = src_array_element;
        self
    }

    /// Set the destination descriptor set for the copy operation
    pub fn set_dst_set(mut self, dst_set: &'a VulkanDescriptorSet) -> Self {
        self.inner.dst_set = dst_set.handle;
        self
    }

    /// Set the destination binding index in the destination descriptor set
    pub fn set_dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }

    /// Set the destination array element index in the destination descriptor set
    pub fn set_dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }

    /// Set the number of descriptors to copy from the source to the destination
    pub fn set_descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
}
