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
                VkDebugUtilsMessageSeverityFlagExt::ErrorExt.into()
            }
            VulkanDebugMessageSeverity::Warning => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningExt
            }
            VulkanDebugMessageSeverity::Info => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoExt
            }
            VulkanDebugMessageSeverity::Verbose => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoExt
                    | VkDebugUtilsMessageSeverityFlagExt::VerboseExt
            }
        }
    }
}
