use crate::FunctionSymbol;
use vulkan::{VkCreateSampler, VkDestroySampler};

mod load;

/// The functions that are used by samplers associated with a device
pub(in crate::gpu::device) struct VulkanSamplerFunctions {
    /// The function to create a sampler
    pub create_sampler: FunctionSymbol<VkCreateSampler>,

    /// The function to destroy a sampler
    pub destroy_sampler: FunctionSymbol<VkDestroySampler>,
}
