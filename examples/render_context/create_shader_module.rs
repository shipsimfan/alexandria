use crate::render_context::RenderContext;
use alexandria::gpu::{VulkanShaderModule, VulkanShaderModuleCode};

impl RenderContext {
    /// Create a new [`VulkanShaderModule`]
    #[allow(unused)]
    pub fn create_shader_module<const N: usize>(
        &mut self,
        code: &VulkanShaderModuleCode<N>,
    ) -> VulkanShaderModule {
        self.device.create_shader_module(code).unwrap()
    }
}
