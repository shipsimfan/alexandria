use crate::gpu::GpuAddress;
use std::marker::PhantomData;

impl<T> GpuAddress<T> {
    /// Create a new [`GpuAddress`]
    pub(in crate::gpu) fn new(address: u64) -> GpuAddress<T> {
        GpuAddress {
            address,
            _marker: PhantomData,
        }
    }
}
