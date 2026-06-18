use crate::gpu::{
    VulkanPipelineVertexInputStateCreateInfo, VulkanVertexInputAttributeDescription,
    VulkanVertexInputBindingDescription,
};

impl<'a> VulkanPipelineVertexInputStateCreateInfo<'a> {
    /// Get the vertex input attribute descriptions
    pub fn attribute_descriptions(&self) -> &'a [VulkanVertexInputAttributeDescription] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.vertex_attribute_descriptions.cast(),
                self.inner.vertex_attribute_description_count as _,
            )
        }
    }

    /// Get the vertex input binding descriptions
    pub fn binding_descriptions(&self) -> &'a [VulkanVertexInputBindingDescription] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.vertex_binding_descriptions.cast(),
                self.inner.vertex_binding_description_count as _,
            )
        }
    }
}
