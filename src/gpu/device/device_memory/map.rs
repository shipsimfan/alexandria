use crate::{
    Error, MemorySize,
    gpu::{VulkanDeviceMemory, VulkanMappedMemory, VulkanMemoryMapFlags},
};

impl VulkanDeviceMemory {
    /// Map the memory to the host
    pub fn map<T, M: Into<MemorySize>, F: Into<VulkanMemoryMapFlags>>(
        self,
        offset: u64,
        size: M,
        flags: F,
    ) -> Result<VulkanMappedMemory<T>, (Error, VulkanDeviceMemory)> {
        VulkanMappedMemory::new(self, offset, size.into(), flags.into())
    }
}
