use crate::gpu::{VulkanPipelineInputAssemblyStateCreateInfo, VulkanPrimitiveTopology};
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineInputAssemblyStateCreateInfo};

impl VulkanPipelineInputAssemblyStateCreateInfo {
    /// Creates a new [`VulkanPipelineInputAssemblyStateCreateInfo`]
    pub fn new(
        topology: VulkanPrimitiveTopology,
        primitive_restart_enable: bool,
    ) -> VulkanPipelineInputAssemblyStateCreateInfo {
        VulkanPipelineInputAssemblyStateCreateInfo {
            inner: VkPipelineInputAssemblyStateCreateInfo {
                topology,
                primitive_restart_enable: if primitive_restart_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                ..Default::default()
            },
        }
    }
}
