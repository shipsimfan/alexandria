use crate::gpu::VulkanDebugMessageSeverity;
use vulkan::ext_debug_utils::VkDebugUtilsMessageSeverityFlagExt;

impl VulkanDebugMessageSeverity {
    /// Convert a Vulkan severity into a [`VulkanDebugMessageSeverity`]
    pub(in crate::gpu::instance::debug_messenger) fn from_vk(
        vk: VkDebugUtilsMessageSeverityFlagExt,
    ) -> VulkanDebugMessageSeverity {
        match vk {
            VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt => VulkanDebugMessageSeverity::Error,
            VkDebugUtilsMessageSeverityFlagExt::WarningBitExt => {
                VulkanDebugMessageSeverity::Warning
            }
            VkDebugUtilsMessageSeverityFlagExt::InfoBitExt => VulkanDebugMessageSeverity::Info,
            _ => VulkanDebugMessageSeverity::Verbose,
        }
    }
}
