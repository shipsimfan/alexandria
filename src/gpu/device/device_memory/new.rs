use crate::{
    Error, MemorySize, Result,
    gpu::{VulkanDevice, VulkanDeviceMemory},
};
use std::ptr::null;
use vulkan::{VkDeviceMemory, VkMemoryAllocateInfo, try_vulkan};

impl VulkanDeviceMemory {
    /// Create a new [`VulkanDeviceMemory`]
    pub(in crate::gpu::device) fn new(
        allocation_size: MemorySize,
        memory_type_index: usize,
        device: &VulkanDevice,
    ) -> Result<VulkanDeviceMemory> {
        let mut handle = VkDeviceMemory::null();
        let allocate_info = VkMemoryAllocateInfo {
            allocation_size: *allocation_size,
            memory_type_index: memory_type_index as _,
            ..Default::default()
        };

        try_vulkan!((device.functions().device_memory.allocate_memory)(
            device.handle(),
            &allocate_info,
            null(),
            &mut handle,
        ))
        .map(|_| VulkanDeviceMemory {
            handle,
            device: device.clone(),
        })
        .map_err(|error| Error::new_with("unable to allocate device memory", error))
    }
}
