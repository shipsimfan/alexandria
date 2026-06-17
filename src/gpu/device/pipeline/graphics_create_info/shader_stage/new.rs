use crate::gpu::{
    VulkanPipelineShaderStageCreateFlags, VulkanPipelineShaderStageCreateInfo, VulkanShaderModule,
    VulkanShaderStageFlag, VulkanSpecializationInfo,
};
use std::{ffi::CStr, marker::PhantomData, ptr::null};
use vulkan::VkPipelineShaderStageCreateInfo;

impl<'a> VulkanPipelineShaderStageCreateInfo<'a> {
    /// Create a new [`VulkanPipelineShaderStageCreateInfo`]
    pub fn new<F: Into<VulkanPipelineShaderStageCreateFlags>>(
        flags: F,
        stage: VulkanShaderStageFlag,
        module: &'a VulkanShaderModule,
        name: &'a CStr,
        specialization_info: Option<&'a VulkanSpecializationInfo<'a>>,
    ) -> VulkanPipelineShaderStageCreateInfo<'a> {
        VulkanPipelineShaderStageCreateInfo {
            inner: VkPipelineShaderStageCreateInfo {
                flags: flags.into(),
                stage,
                module: module.handle(),
                name: name.as_ptr(),
                specialization_info: specialization_info.map_or(null(), |info| info.as_ptr()),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
