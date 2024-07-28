use super::log_callback;
use std::borrow::Cow;
use util::Severity;
use vulkan::{
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagBitsEXT,
    VkDebugUtilsMessengerCreateInfoEXT,
};

/// Creates [`VkDebugUtilsMessengerCreateInfoEXT`] for `log` function
pub(in crate::instance) fn debug_messenger_create_info(
    callback: *const Box<dyn Fn(Severity, &str, Vec<Cow<str>>)>,
) -> VkDebugUtilsMessengerCreateInfoEXT {
    VkDebugUtilsMessengerCreateInfoEXT {
        message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT::VerboseBitEXT as u32
            | VkDebugUtilsMessageSeverityFlagBitsEXT::InfoBitEXT as u32
            | VkDebugUtilsMessageSeverityFlagBitsEXT::WarningBitEXT as u32
            | VkDebugUtilsMessageSeverityFlagBitsEXT::ErrorBitEXT as u32,
        message_type: VkDebugUtilsMessageTypeFlagBitsEXT::AddressBindingBitEXT as u32
            | VkDebugUtilsMessageTypeFlagBitsEXT::GeneralBitEXT as u32
            | VkDebugUtilsMessageTypeFlagBitsEXT::PerformanceBitEXT as u32
            | VkDebugUtilsMessageTypeFlagBitsEXT::ValidationBitEXT as u32,
        user_callback: log_callback,
        user_data: callback as _,
        ..Default::default()
    }
}
