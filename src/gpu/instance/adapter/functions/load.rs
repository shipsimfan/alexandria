use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanAdapterFunctions, load_instance_function},
};
use vulkan::{
    VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES, VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES,
    VK_GET_PHYSICAL_DEVICE_PROPERTIES, VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES, VkInstance,
};

impl VulkanAdapterFunctions {
    /// Load all the required adapter functions
    pub fn load(context: &GpuSubsystem, instance: VkInstance) -> Result<VulkanAdapterFunctions> {
        Ok(VulkanAdapterFunctions {
            get_physical_device_properties: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_PROPERTIES
            )?,
            get_physical_device_memory_properties: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES
            )?,
            get_physical_device_queue_family_properties: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES
            )?,
            enumerate_device_extension_properties: load_instance_function!(
                context,
                instance,
                VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES
            )?,
        })
    }
}
