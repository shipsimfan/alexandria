use vulkan::VkDestroyInstance;

mod load;

/// The functions loaded for a specific graphics instance
pub(in crate::instance) struct GraphicsInstanceFunctions {
    /// The function used to destroy the instance
    pub destroy_instance: VkDestroyInstance,
}
