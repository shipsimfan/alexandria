use crate::FunctionSymbol;
use vulkan::{VkCreateShaderModule, VkDestroyShaderModule};

mod load;

/// The functions that are used by shader modules associated with a device
pub(in crate::gpu::device) struct VulkanShaderModuleFunctions {
    /// The function to create a shader module
    pub create_shader_module: FunctionSymbol<VkCreateShaderModule>,

    /// The function to destroy a shader module
    pub destroy_semaphore: FunctionSymbol<VkDestroyShaderModule>,
}
