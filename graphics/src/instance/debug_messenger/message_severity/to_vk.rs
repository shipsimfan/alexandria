use crate::GraphicsDebugMessageSeverity;
use vulkan::ext_debug_utils::{
    VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageSeverityFlagsExt,
};

impl GraphicsDebugMessageSeverity {
    /// Convert a [`GraphicsDebugMessageSeverity`] into a Vulkan severity set
    pub(in crate::instance::debug_messenger) fn to_vk(
        &self,
    ) -> VkDebugUtilsMessageSeverityFlagsExt {
        match self {
            GraphicsDebugMessageSeverity::Error => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt.into()
            }
            GraphicsDebugMessageSeverity::Warning => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
            }
            GraphicsDebugMessageSeverity::Info => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoBitExt
            }
            GraphicsDebugMessageSeverity::Verbose => {
                VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::WarningBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::InfoBitExt
                    | VkDebugUtilsMessageSeverityFlagExt::VerboseBitExt
            }
        }
    }
}
