use crate::gpu::{VulkanShaderModuleCode, device::shader_module::code::Align4};
use std::ffi::CStr;

impl<const N: usize> VulkanShaderModuleCode<N> {
    /// Create a new [`VulkanShaderModuleCode`]
    pub const fn new(
        file_name: &'static str,
        entry_points: &'static [&'static CStr],
        code: [u8; N],
    ) -> VulkanShaderModuleCode<N> {
        VulkanShaderModuleCode {
            file_name,
            code: Align4::new(code),
            entry_points,
        }
    }
}
