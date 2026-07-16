use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanDeviceMemoryFunctions, load_device_function},
};
use vulkan::{VK_ALLOCATE_MEMORY, VK_FREE_MEMORY, VK_MAP_MEMORY, VK_UNMAP_MEMORY, VkDevice};

impl VulkanDeviceMemoryFunctions {
    /// Load all the required device memory functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanDeviceMemoryFunctions> {
        Ok(VulkanDeviceMemoryFunctions {
            allocate_memory: load_device_function!(instance, device, VK_ALLOCATE_MEMORY)?,
            free_memory: load_device_function!(instance, device, VK_FREE_MEMORY)?,
            map_memory: load_device_function!(instance, device, VK_MAP_MEMORY)?,
            unmap_memory: load_device_function!(instance, device, VK_UNMAP_MEMORY)?,
        })
    }
}
