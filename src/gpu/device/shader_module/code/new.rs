use crate::gpu::{VulkanShaderModuleCode, device::shader_module::code::Align4};
use std::ffi::CStr;

impl<const N: usize> VulkanShaderModuleCode<N> {
    /// Create a new [`VulkanShaderModuleCode`]
    pub const fn new(
        code: [u8; N],
        entry_points: &'static [&'static CStr],
    ) -> VulkanShaderModuleCode<N> {
        VulkanShaderModuleCode {
            code: Align4::new(code),
            entry_points,
        }
    }
}
