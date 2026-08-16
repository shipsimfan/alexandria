use crate::{
    MemorySize, Result,
    gpu::{VulkanDevice, VulkanDeviceMemory, VulkanMemoryAllocateFlags},
};
use std::ptr::{addr_of, null};
use vulkan::VkMemoryAllocateFlagsInfo;

impl VulkanDevice {
    /// Allocate a block of memory on the GPU
    pub fn allocate_memory<M: Into<MemorySize>>(
        &self,
        allocation_size: M,
        memory_type_index: usize,
    ) -> Result<VulkanDeviceMemory> {
        VulkanDeviceMemory::new(allocation_size.into(), memory_type_index, null(), self)
    }

    /// Allocate a block of memory on the GPU
    pub fn allocate_memory_flags<M: Into<MemorySize>, F: Into<VulkanMemoryAllocateFlags>>(
        &self,
        allocation_size: M,
        memory_type_index: usize,
        flags: F,
    ) -> Result<VulkanDeviceMemory> {
        let flags = VkMemoryAllocateFlagsInfo {
            flags: flags.into(),
            ..Default::default()
        };

        VulkanDeviceMemory::new(
            allocation_size.into(),
            memory_type_index,
            addr_of!(flags).cast(),
            self,
        )
    }
}
