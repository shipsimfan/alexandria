use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanShaderModule, VulkanShaderModuleCode},
};
use std::ptr::null;
use vulkan::{VkShaderModule, VkShaderModuleCreateInfo, try_vulkan};

impl VulkanShaderModule {
    /// Create a new [`VulkanShaderModule`]
    pub(in crate::gpu::device) fn new<const N: usize>(
        device: &VulkanDevice,
        code: &VulkanShaderModuleCode<N>,
    ) -> Result<VulkanShaderModule> {
        let code = code.code();
        let create_info = VkShaderModuleCreateInfo {
            code_size: code.len(),
            code: code.as_ptr().cast(),
            ..Default::default()
        };

        let mut handle = VkShaderModule::null();
        try_vulkan!((device.functions().shader_module.create_shader_module)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to create a shader module", error))?;

        Ok(VulkanShaderModule {
            handle,
            device: device.clone(),
        })
    }
}
