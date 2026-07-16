use crate::FunctionSymbol;
use vulkan::{VkAllocateMemory, VkFreeMemory, VkMapMemory, VkUnmapMemory};

mod load;

/// The functions that are used by device memory associated with a device
pub(in crate::gpu::device) struct VulkanDeviceMemoryFunctions {
    /// The function to allocate device memory
    pub allocate_memory: FunctionSymbol<VkAllocateMemory>,

    /// The function to free device memory
    pub free_memory: FunctionSymbol<VkFreeMemory>,

    /// The function map memory to the host
    pub map_memory: FunctionSymbol<VkMapMemory>,

    /// The function to unmap memory from the host
    pub unmap_memory: FunctionSymbol<VkUnmapMemory>,
}
