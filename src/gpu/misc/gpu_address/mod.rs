use std::marker::PhantomData;

mod add;
mod clone;
mod debug;
mod eq;
mod new;
mod pointer;

/// An address in GPU memory
#[repr(C)]
#[derive(Copy)]
pub struct GpuAddress<T> {
    /// The actual value of the GPU address
    address: u64,

    /// The type being pointed to by this GPU address
    _marker: PhantomData<T>,
}
