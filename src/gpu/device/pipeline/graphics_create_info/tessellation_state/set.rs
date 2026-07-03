use crate::gpu::VulkanPipelineTessellationStateCreateInfo;

impl VulkanPipelineTessellationStateCreateInfo {
    /// Set the patch control points for the tessellation state create info
    pub fn set_patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.inner.patch_control_points = patch_control_points;
        self
    }
}
