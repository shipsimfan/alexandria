use crate::gpu::{
    VulkanPipelineVertexInputStateCreateInfo, VulkanVertexInputAttributeDescription,
    VulkanVertexInputBindingDescription,
};
use std::marker::PhantomData;
use vulkan::VkPipelineVertexInputStateCreateInfo;

impl<'a> VulkanPipelineVertexInputStateCreateInfo<'a> {
    /// Creates a new [`VulkanPipelineVertexInputStateCreateInfo`]
    pub fn new(
        vertex_attribute_descriptions: &'a [VulkanVertexInputAttributeDescription],
        vertex_binding_descriptions: &'a [VulkanVertexInputBindingDescription],
    ) -> VulkanPipelineVertexInputStateCreateInfo<'a> {
        VulkanPipelineVertexInputStateCreateInfo {
            inner: VkPipelineVertexInputStateCreateInfo {
                vertex_attribute_description_count: vertex_attribute_descriptions.len() as _,
                vertex_attribute_descriptions: vertex_attribute_descriptions.as_ptr().cast(),
                vertex_binding_description_count: vertex_binding_descriptions.len() as _,
                vertex_binding_descriptions: vertex_binding_descriptions.as_ptr().cast(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
