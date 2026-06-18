use crate::gpu::VulkanPipelineTessellationStateCreateInfo;

impl VulkanPipelineTessellationStateCreateInfo {
    /// Get the patch control points for the tessellation state create info
    pub fn patch_control_points(&self) -> u32 {
        self.inner.patch_control_points
    }
}
