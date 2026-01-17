use vulkan::VkDestroyDevice;

mod load;

/// The functions loaded for a specific graphics device
pub(in crate::device) struct GraphicsDeviceFunctions {
    /// The function used to destroy the device
    pub destroy_device: VkDestroyDevice,
}
