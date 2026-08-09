use crate::FunctionSymbol;
use vulkan::{VkCreateDescriptorSetLayout, VkDestroyDescriptorSetLayout};

mod load;

/// The functions that are used by descriptor set layouts
pub(in crate::gpu::device) struct VulkanDescriptorSetLayoutFunctions {
    /// The function to create a descriptor set layout
    pub create_descriptor_set_layout: FunctionSymbol<VkCreateDescriptorSetLayout>,

    /// The function to destroy a descriptor set layout
    pub destroy_descriptor_set_layout: FunctionSymbol<VkDestroyDescriptorSetLayout>,
}
