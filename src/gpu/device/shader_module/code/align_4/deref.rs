use crate::gpu::device::shader_module::code::Align4;
use std::ops::Deref;

impl<const N: usize> Deref for Align4<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
