mod deref;
mod new;

/// A helper type to ensure that the shader code is aligned to 4 bytes, as required by Vulkan
#[repr(C, align(4))]
pub(in crate::gpu::device::shader_module::code) struct Align4<const N: usize> {
    /// The contained data
    data: [u8; N],
}
