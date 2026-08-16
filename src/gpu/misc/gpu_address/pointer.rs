use crate::gpu::GpuAddress;
use std::fmt::Pointer;

impl<T> Pointer for GpuAddress<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.address as *const T).fmt(f)
    }
}
