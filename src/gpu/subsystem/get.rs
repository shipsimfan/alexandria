use crate::gpu::{GpuSubsystem, subsystem::functions::GlobalVulkanFunctions};

impl GpuSubsystem {
    /// Get the global Vulkan functions
    pub(in crate::gpu) fn functions(&self) -> &GlobalVulkanFunctions {
        self.inner.functions()
    }
}
