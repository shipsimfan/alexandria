use crate::gpu::GpuAddress;
use std::fmt::Debug;

impl<T> Debug for GpuAddress<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GpuAddress({:#x})", self.address)
    }
}
