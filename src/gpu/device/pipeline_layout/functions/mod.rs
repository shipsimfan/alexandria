use crate::FunctionSymbol;
use vulkan::{VkCreatePipelineLayout, VkDestroyPipelineLayout};

mod load;

/// The functions that are used by pipeline layouts associated with a device
pub(in crate::gpu::device) struct VulkanPipelineLayoutFunctions {
    /// The function to create a pipeline layout
    pub create_pipeline_layout: FunctionSymbol<VkCreatePipelineLayout>,

    /// The function to destroy a pipeline layout
    pub destroy_pipeline_layout: FunctionSymbol<VkDestroyPipelineLayout>,
}
