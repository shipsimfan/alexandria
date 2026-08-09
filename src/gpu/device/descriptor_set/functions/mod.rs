use crate::FunctionSymbol;
use vulkan::{VkAllocateDescriptorSets, VkFreeDescriptorSets};

mod load;

/// The functions that are used by descriptor sets
pub(in crate::gpu::device) struct VulkanDescriptorSetFunctions {
    /// The function to allocate descriptor sets from a descriptor pool
    pub allocate_descriptor_sets: FunctionSymbol<VkAllocateDescriptorSets>,

    /// The function to free descriptor sets back to a descriptor pool
    pub free_descriptor_sets: FunctionSymbol<VkFreeDescriptorSets>,
}
