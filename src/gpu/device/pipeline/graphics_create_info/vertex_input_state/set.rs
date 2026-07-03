use crate::gpu::{
    VulkanPipelineVertexInputStateCreateInfo, VulkanVertexInputAttributeDescription,
    VulkanVertexInputBindingDescription,
};

impl<'a> VulkanPipelineVertexInputStateCreateInfo<'a> {
    /// Set the vertex input attribute descriptions
    pub fn set_attribute_descriptions(
        mut self,
        attribute_descriptions: &'a [VulkanVertexInputAttributeDescription],
    ) -> Self {
        self.inner.vertex_attribute_descriptions = attribute_descriptions.as_ptr() as _;
        self.inner.vertex_attribute_description_count = attribute_descriptions.len() as _;
        self
    }

    /// Set the vertex input binding descriptions
    pub fn set_binding_descriptions(
        mut self,
        binding_descriptions: &'a [VulkanVertexInputBindingDescription],
    ) -> Self {
        self.inner.vertex_binding_descriptions = binding_descriptions.as_ptr() as _;
        self.inner.vertex_binding_description_count = binding_descriptions.len() as _;
        self
    }
}
