use crate::instance::GraphicsInstanceInner;
use callback::debug_message_trampoline;
use std::sync::Arc;
use vulkan::ext_debug_utils::VkDebugUtilsMessengerExt;

mod callback;
mod functions;
mod message_severity;

mod drop;
mod new;

pub use callback::GraphicsDebugMessengerCallback;
pub use message_severity::GraphicsDebugMessageSeverity;

pub(in crate::instance) use functions::GraphicsDebugMessengerFunctions;

/// An object which calls a callback when a debug message is emitted from Vulkan
pub struct GraphicsDebugMessenger<C: GraphicsDebugMessengerCallback> {
    /// The callback for Vulkan debug messages
    #[allow(unused)]
    callback: Box<C>,

    /// The handle to the underlying debug messenger
    handle: VkDebugUtilsMessengerExt,

    /// The instance this debug messenger was created for
    instance: Arc<GraphicsInstanceInner>,
}
