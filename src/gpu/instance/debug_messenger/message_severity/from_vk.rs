use crate::gpu::VulkanDebugMessageSeverity;
use vulkan::ext_debug_utils::VkDebugUtilsMessageSeverityFlagExt;

impl VulkanDebugMessageSeverity {
    /// Convert a Vulkan severity into a [`VulkanDebugMessageSeverity`]
    pub(in crate::gpu::instance::debug_messenger) fn from_vk(
        vk: VkDebugUtilsMessageSeverityFlagExt,
    ) -> VulkanDebugMessageSeverity {
        match vk {
            VkDebugUtilsMessageSeverityFlagExt::ErrorExt => VulkanDebugMessageSeverity::Error,
            VkDebugUtilsMessageSeverityFlagExt::WarningExt => VulkanDebugMessageSeverity::Warning,
            VkDebugUtilsMessageSeverityFlagExt::InfoExt => VulkanDebugMessageSeverity::Info,
            _ => VulkanDebugMessageSeverity::Verbose,
        }
    }
}
