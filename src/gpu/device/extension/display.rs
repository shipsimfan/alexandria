use crate::gpu::VulkanDeviceExtension;

impl std::fmt::Display for VulkanDeviceExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulkanDeviceExtension::Swapchain => write!(f, "swapchain"),
            VulkanDeviceExtension::ExtendedDynamicState => write!(f, "extended dynamic state"),
        }
    }
}
