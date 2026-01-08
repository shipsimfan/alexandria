use std::ffi::{CStr, c_void};

use crate::GraphicsDebugMessageSeverity;
use vulkan::{
    VK_FALSE, VkBool32,
    ext_debug_utils::{
        VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageTypeFlagsExt,
        VkDebugUtilsMessengerCallbackDataExt,
    },
};

/// An object which can be used as a callback for Vulkan debug messages
pub trait GraphicsDebugMessengerCallback {
    /// Called when a debug message is emitted from Vulkan
    fn message(&self, message: &str, severity: GraphicsDebugMessageSeverity);
}

/// The function to pass to the raw debug message callback
pub(in crate::instance::debug_messenger) extern "system" fn debug_message_trampoline<
    C: GraphicsDebugMessengerCallback,
>(
    message_severity: VkDebugUtilsMessageSeverityFlagExt,
    _: VkDebugUtilsMessageTypeFlagsExt,
    callback_data: *const VkDebugUtilsMessengerCallbackDataExt,
    user_data: *mut c_void,
) -> VkBool32 {
    let data: &C = unsafe { &*user_data.cast() };
    let callback_data = unsafe { &*callback_data };

    let message = unsafe { CStr::from_ptr(callback_data.message) }.to_string_lossy();
    let severity = GraphicsDebugMessageSeverity::from_vk(message_severity);

    data.message(&message, severity);

    VK_FALSE
}
