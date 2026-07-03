use crate::gpu::{VulkanFormat, VulkanVertexInputAttributeDescription};

impl VulkanVertexInputAttributeDescription {
    /// Get the location of the vertex input attribute
    pub fn location(&self) -> u32 {
        self.inner.location
    }

    /// Get the binding of the vertex input attribute
    pub fn binding(&self) -> u32 {
        self.inner.binding
    }

    /// Get the format of the vertex input attribute
    pub fn format(&self) -> VulkanFormat {
        self.inner.format
    }

    /// Get the offset of the vertex input attribute
    pub fn offset(&self) -> u32 {
        self.inner.offset
    }
}
