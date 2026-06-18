use crate::gpu::{VulkanVertexInputBindingDescription, VulkanVertexInputRate};

impl VulkanVertexInputBindingDescription {
    /// Get the binding of the vertex input binding description
    pub fn binding(&self) -> u32 {
        self.inner.binding
    }

    /// Get the stride of the vertex input binding description
    pub fn stride(&self) -> u32 {
        self.inner.stride
    }

    /// Get the input rate of the vertex input binding description
    pub fn input_rate(&self) -> VulkanVertexInputRate {
        self.inner.input_rate
    }
}
