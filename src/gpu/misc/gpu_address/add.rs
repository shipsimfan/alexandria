use crate::gpu::GpuAddress;

impl<T> GpuAddress<T> {
    /// Advance the address by a given number of elements of type `T`
    pub fn add(&self, count: usize) -> GpuAddress<T> {
        GpuAddress::new(self.address + (count * std::mem::size_of::<T>()) as u64)
    }
}
