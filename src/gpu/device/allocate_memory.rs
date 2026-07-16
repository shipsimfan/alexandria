use crate::{
    MemorySize, Result,
    gpu::{VulkanDevice, VulkanDeviceMemory},
};

impl VulkanDevice {
    /// Allocate a block of memory on the GPU
    pub fn allocate_memory<M: Into<MemorySize>>(
        &self,
        allocation_size: M,
        memory_type_index: usize,
    ) -> Result<VulkanDeviceMemory> {
        VulkanDeviceMemory::new(allocation_size.into(), memory_type_index, self)
    }
}
