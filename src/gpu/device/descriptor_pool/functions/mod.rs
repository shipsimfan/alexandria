use crate::FunctionSymbol;
use vulkan::{VkCreateDescriptorPool, VkDestroyDescriptorPool};

mod load;

/// The functions that are used by descriptor pools
pub(in crate::gpu::device) struct VulkanDescriptorPoolFunctions {
    /// The function to create a descriptor pool
    pub create_descriptor_pool: FunctionSymbol<VkCreateDescriptorPool>,

    /// The function to destroy a descriptor pool
    pub destroy_descriptor_pool: FunctionSymbol<VkDestroyDescriptorPool>,
}
