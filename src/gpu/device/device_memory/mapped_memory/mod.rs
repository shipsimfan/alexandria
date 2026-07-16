use crate::gpu::VulkanDeviceMemory;

mod deref;
mod drop;
mod new;
mod unmap;

/// Memory mapped to the host by Vulkan
#[must_use]
pub struct VulkanMappedMemory<T> {
    /// The underlying device memory that is mapped to the host
    memory: Option<VulkanDeviceMemory>,

    /// The pointer to the mapped memory on the host
    ptr: *mut T,

    /// The number of elements of type `T` that are mapped to the host
    length: usize,
}
