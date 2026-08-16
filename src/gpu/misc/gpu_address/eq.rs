use crate::gpu::GpuAddress;

impl<T> PartialEq for GpuAddress<T> {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl<T> Eq for GpuAddress<T> {}
