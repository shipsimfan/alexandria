use vulkan::VkObjectType;

/// An object that is nameable
pub trait VulkanNameable {
    /// The type of the object
    const OBJECT_TYPE: VkObjectType;

    /// Get the handle of the Vulkan object
    fn handle(&self) -> u64;
}
