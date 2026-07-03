use crate::gpu::{VulkanVertexInputBindingDescription, VulkanVertexInputRate};

impl VulkanVertexInputBindingDescription {
    /// Set the binding of the vertex input binding description
    pub fn set_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    /// Set the stride of the vertex input binding description
    pub fn set_stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }

    /// Set the input rate of the vertex input binding description
    pub fn set_input_rate(mut self, input_rate: VulkanVertexInputRate) -> Self {
        self.inner.input_rate = input_rate;
        self
    }
}
