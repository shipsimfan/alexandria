use std::rc::Rc;
use vk::DebugUtilsMessenger;

mod new;
mod physical_device;

pub use new::InstanceCreateError;
pub use physical_device::{EnumeratePhysicalDevicesError, PhysicalDevice};

/// The instance of this library for the application to interact with
pub struct Instance {
    /// The Vulkan instance
    instance: Rc<vk::Instance>,

    /// The Vulkan debug messenger
    #[allow(unused)]
    debug_messenger: Option<DebugUtilsMessenger>,
}
