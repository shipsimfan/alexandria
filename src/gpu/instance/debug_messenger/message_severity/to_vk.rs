use crate::gpu::VulkanDebugMessageSeverity;
use vulkan::ext_debug_utils::{
    VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageSeverityFlagsExt,
};

impl VulkanDebugMessageSeverity {
    /// Convert a [`VulkanDebugMessageSeverity`] into a Vulkan severity set
    pub(in crate::gpu::instance::debug_messenger) fn to_vk(
        &self,
    ) -> VkDebugUtilsMessageSeverityFlagsExt {
        match self {
            VulkanDebugMessageSeverity::Error => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt.into()
            }
            VulkanDebugMessageSeverity::Warning => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
            }
            VulkanDebugMessageSeverity::Info => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoBitExt
            }
            VulkanDebugMessageSeverity::Verbose => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::VerboseBitExt
            }
        }
    }
}
