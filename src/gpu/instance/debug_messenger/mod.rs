use crate::gpu::VulkanInstance;
use callback::debug_message_trampoline;
use vulkan::ext_debug_utils::VkDebugUtilsMessengerExt;

mod callback;
mod functions;

mod drop;
mod new;

pub use callback::*;

pub(in crate::gpu::instance) use functions::*;

/// An object which calls a callback when a debug message is emitted from Vulkan
pub struct VulkanDebugMessenger<C: VulkanDebugMessengerCallback> {
    /// The callback for Vulkan debug messages
    #[allow(unused)]
    callback: Box<C>,

    /// The handle to the underlying debug messenger
    handle: VkDebugUtilsMessengerExt,

    /// The instance this debug messenger was created for
    instance: VulkanInstance,
}
