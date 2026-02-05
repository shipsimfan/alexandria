use crate::gpu::instance::VulkanInstanceInner;
use callback::debug_message_trampoline;
use std::sync::Arc;
use vulkan::ext_debug_utils::VkDebugUtilsMessengerExt;

mod callback;
mod functions;
mod message_severity;

mod drop;
mod new;

pub use callback::VulkanDebugMessengerCallback;
pub use message_severity::VulkanDebugMessageSeverity;

pub(in crate::gpu::instance) use functions::VulkanDebugMessengerFunctions;

/// An object which calls a callback when a debug message is emitted from Vulkan
pub struct VulkanDebugMessenger<C: VulkanDebugMessengerCallback> {
    /// The callback for Vulkan debug messages
    #[allow(unused)]
    callback: Box<C>,

    /// The handle to the underlying debug messenger
    handle: VkDebugUtilsMessengerExt,

    /// The instance this debug messenger was created for
    instance: Arc<VulkanInstanceInner>,
}
