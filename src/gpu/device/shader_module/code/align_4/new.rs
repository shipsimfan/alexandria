use crate::gpu::device::shader_module::code::Align4;

impl<const N: usize> Align4<N> {
    /// Create a new [`Align4`]
    pub const fn new(data: [u8; N]) -> Align4<N> {
        Align4 { data }
    }
}
