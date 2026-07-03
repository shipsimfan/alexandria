use align_4::Align4;
use std::ffi::CStr;

mod align_4;

mod get;
mod new;

/// The code describing a shader module for a Vulkan device
pub struct VulkanShaderModuleCode<const N: usize> {
    /// The entry point names of the shader module
    entry_points: &'static [&'static CStr],

    /// The code of the shader module
    code: Align4<N>,
}
