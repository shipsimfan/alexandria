use std::{borrow::Cow, rc::Rc};
use util::Severity;
use vk::DebugUtilsMessenger;

mod new;

pub use new::InstanceCreateError;

/// The instance of this library for the application to interact with
pub struct Instance {
    /// The Vulkan instance
    instance: Rc<vk::Instance>,

    /// The Vulkan debug messenger
    #[allow(unused)]
    debug_messenger: Option<DebugUtilsMessenger>,
}
