use crate::gpu::GpuAddress;

impl<T> Clone for GpuAddress<T> {
    fn clone(&self) -> Self {
        GpuAddress::new(self.address)
    }
}

impl<T> Copy for GpuAddress<T> {}