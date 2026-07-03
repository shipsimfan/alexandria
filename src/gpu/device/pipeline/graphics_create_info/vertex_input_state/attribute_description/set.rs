use crate::gpu::{VulkanFormat, VulkanVertexInputAttributeDescription};

impl VulkanVertexInputAttributeDescription {
    /// Set the location of the vertex input attribute
    pub fn set_location(mut self, location: u32) -> Self {
        self.inner.location = location;
        self
    }

    /// Set the binding of the vertex input attribute
    pub fn set_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    /// Set the format of the vertex input attribute
    pub fn set_format(mut self, format: VulkanFormat) -> Self {
        self.inner.format = format;
        self
    }

    /// Set the offset of the vertex input attribute
    pub fn set_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}
