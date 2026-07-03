use crate::{
    Result,
    gpu::{VulkanDevice, VulkanShaderModule, VulkanShaderModuleCode},
};

impl VulkanDevice {
    /// Create a new [`VulkanShaderModule`]
    pub fn create_shader_module<const N: usize>(
        &self,
        code: &VulkanShaderModuleCode<N>,
    ) -> Result<VulkanShaderModule> {
        VulkanShaderModule::new(self, code)
    }
}
