use crate::gpu::{
    VulkanPipelineShaderStageCreateFlags, VulkanPipelineShaderStageCreateInfo,
    VulkanShaderStageFlag, VulkanSpecializationInfo,
};
use std::ffi::CStr;

impl<'a> VulkanPipelineShaderStageCreateInfo<'a> {
    /// Get the flags of the shader stage
    pub fn flags(&self) -> VulkanPipelineShaderStageCreateFlags {
        self.inner.flags
    }

    /// Get the shader stage
    pub fn stage(&self) -> VulkanShaderStageFlag {
        self.inner.stage
    }

    /// Get the name of the entry point function
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.name) }
    }

    /// Get the specialization info for the shader stage, if any
    pub fn specialization_info(&self) -> Option<&'a VulkanSpecializationInfo<'a>> {
        unsafe {
            self.inner
                .specialization_info
                .cast::<VulkanSpecializationInfo>()
                .as_ref()
        }
    }
}
