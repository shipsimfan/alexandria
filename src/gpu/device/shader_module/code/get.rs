use crate::gpu::device::shader_module::code::VulkanShaderModuleCode;
use std::{ffi::CStr, ops::Deref};

impl<const N: usize> VulkanShaderModuleCode<N> {
    /// Get the name of the file the shader module was created from
    pub fn file_name(&self) -> &'static str {
        self.file_name
    }

    /// Get the entry point names of the shader module
    pub fn entry_points(&self) -> &'static [&'static CStr] {
        self.entry_points
    }

    /// Get the code of the shader module
    pub(in crate::gpu::device::shader_module) fn code(&self) -> &[u8] {
        self.code.deref()
    }
}
