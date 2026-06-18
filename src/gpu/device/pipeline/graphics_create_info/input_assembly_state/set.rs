use crate::gpu::{VulkanPipelineInputAssemblyStateCreateInfo, VulkanPrimitiveTopology};
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanPipelineInputAssemblyStateCreateInfo {
    /// Set the primitive topology of the input assembly state
    pub fn set_primitive_topology(&mut self, topology: VulkanPrimitiveTopology) {
        self.inner.topology = topology;
    }

    /// Set whether primitive restart is enabled in the input assembly state
    pub fn set_primitive_restart_enable(&mut self, enable: bool) {
        self.inner.primitive_restart_enable = if enable { VK_TRUE } else { VK_FALSE };
    }
}
