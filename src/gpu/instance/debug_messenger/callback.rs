use crate::gpu::{VulkanDebugMessageSeverityFlag, VulkanDebugMessageTypeFlags};
use std::ffi::{CStr, c_void};
use vulkan::{VK_FALSE, VkBool32, ext_debug_utils::VkDebugUtilsMessengerCallbackDataExt};

/// An object which can be used as a callback for Vulkan debug messages
pub trait VulkanDebugMessengerCallback {
    /// Called when a debug message is emitted from Vulkan
    fn message(
        &self,
        message: &str,
        severity: VulkanDebugMessageSeverityFlag,
        r#type: VulkanDebugMessageTypeFlags,
    );
}

/// The function to pass to the raw debug message callback
pub(in crate::gpu::instance::debug_messenger) extern "system" fn debug_message_trampoline<
    C: VulkanDebugMessengerCallback,
>(
    message_severity: VulkanDebugMessageSeverityFlag,
    message_type: VulkanDebugMessageTypeFlags,
    callback_data: *const VkDebugUtilsMessengerCallbackDataExt,
    user_data: *mut c_void,
) -> VkBool32 {
    let data: &C = unsafe { &*user_data.cast() };
    let callback_data = unsafe { &*callback_data };

    let message = unsafe { CStr::from_ptr(callback_data.message) }.to_string_lossy();

    data.message(&message, message_severity, message_type);

    VK_FALSE
}
