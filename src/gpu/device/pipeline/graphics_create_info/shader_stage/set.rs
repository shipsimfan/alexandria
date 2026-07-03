use crate::gpu::{
    VulkanPipelineShaderStageCreateFlags, VulkanPipelineShaderStageCreateInfo, VulkanShaderModule,
    VulkanShaderStageFlag, VulkanSpecializationInfo,
};
use std::{ffi::CStr, ptr::null};

impl<'a> VulkanPipelineShaderStageCreateInfo<'a> {
    /// Set the flags for the shader stage
    pub fn set_flags<F: Into<VulkanPipelineShaderStageCreateFlags>>(mut self, flags: F) -> Self {
        self.inner.flags = flags.into();
        self
    }

    /// Set the shader stage
    pub fn set_stage(mut self, stage: VulkanShaderStageFlag) -> Self {
        self.inner.stage = stage;
        self
    }

    /// Set the shader module for the shader stage
    pub fn set_module(mut self, module: &'a VulkanShaderModule) -> Self {
        self.inner.module = module.handle();
        self
    }

    /// Set the name of the entry point function
    pub fn set_name(mut self, name: &'a CStr) -> Self {
        self.inner.name = name.as_ptr();
        self
    }

    /// Set the specialization info for the shader stage
    pub fn set_specialization_info(
        mut self,
        specialization_info: Option<&'a VulkanSpecializationInfo<'a>>,
    ) -> Self {
        self.inner.specialization_info = specialization_info.map_or(null(), |info| info.as_ptr());
        self
    }
}
