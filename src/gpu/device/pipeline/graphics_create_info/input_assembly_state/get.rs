use crate::gpu::{VulkanPipelineInputAssemblyStateCreateInfo, VulkanPrimitiveTopology};
use vulkan::VK_TRUE;

impl VulkanPipelineInputAssemblyStateCreateInfo {
    /// Get the primitive topology of the input assembly state
    pub fn primitive_topology(&self) -> VulkanPrimitiveTopology {
        self.inner.topology
    }

    /// Get whether primitive restart is enabled in the input assembly state
    pub fn primitive_restart_enable(&self) -> bool {
        self.inner.primitive_restart_enable == VK_TRUE
    }
}
