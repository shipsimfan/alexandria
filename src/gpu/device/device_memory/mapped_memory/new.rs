use crate::{
    Error, MemorySize,
    gpu::{VulkanDeviceMemory, VulkanMappedMemory, VulkanMemoryMapFlags},
};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl<T> VulkanMappedMemory<T> {
    /// Create a new [`VulkanMappedMemory`]
    pub(in crate::gpu::device::device_memory) fn new(
        device_memory: VulkanDeviceMemory,
        offset: u64,
        size: MemorySize,
        flags: VulkanMemoryMapFlags,
    ) -> Result<VulkanMappedMemory<T>, (Error, VulkanDeviceMemory)> {
        let mut ptr = null_mut();
        match try_vulkan!((device_memory.device.functions().device_memory.map_memory)(
            device_memory.device.handle(),
            device_memory.handle,
            offset,
            *size,
            flags,
            &mut ptr
        )) {
            Ok(_) => Ok(VulkanMappedMemory {
                memory: Some(device_memory),
                ptr: ptr.cast(),
                length: (*size as usize) / std::mem::size_of::<T>(),
            }),
            Err(error) => Err((
                Error::new_with("unable to map device memory", error),
                device_memory,
            )),
        }
    }
}
