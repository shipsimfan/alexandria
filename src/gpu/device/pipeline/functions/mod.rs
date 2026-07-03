use crate::FunctionSymbol;
use vulkan::{VkCreateGraphicsPipelines, VkDestroyPipeline};

mod load;

/// The functions that are used by pipelines associated with a device
pub(in crate::gpu::device) struct VulkanPipelineFunctions {
    /// The function to create graphics pipelines
    pub create_graphics_pipelines: FunctionSymbol<VkCreateGraphicsPipelines>,

    /// The function to destroy a pipeline
    pub destroy_pipeline: FunctionSymbol<VkDestroyPipeline>,
}
