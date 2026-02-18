use crate::gpu::subsystem::{GpuSubsystemInner, functions::GlobalVulkanFunctions};

impl GpuSubsystemInner {
    /// Get the global Vulkan functions
    pub fn functions(&self) -> &GlobalVulkanFunctions {
        &self.functions
    }
}
