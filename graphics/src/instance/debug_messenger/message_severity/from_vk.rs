use crate::GraphicsDebugMessageSeverity;
use vulkan::ext_debug_utils::VkDebugUtilsMessageSeverityFlagExt;

impl GraphicsDebugMessageSeverity {
    /// Convert a Vulkan severity into a [`GraphicsDebugMessageSeverity`]
    pub(in crate::instance::debug_messenger) fn from_vk(
        vk: VkDebugUtilsMessageSeverityFlagExt,
    ) -> GraphicsDebugMessageSeverity {
        match vk {
            VkDebugUtilsMessageSeverityFlagExt::ErrorBitExt => GraphicsDebugMessageSeverity::Error,
            VkDebugUtilsMessageSeverityFlagExt::WarningBitExt => {
                GraphicsDebugMessageSeverity::Warning
            }
            VkDebugUtilsMessageSeverityFlagExt::InfoBitExt => GraphicsDebugMessageSeverity::Info,
            _ => GraphicsDebugMessageSeverity::Verbose,
        }
    }
}
