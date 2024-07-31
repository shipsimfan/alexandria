use std::sync::Arc;
use vk::DebugUtilsMessenger;

mod new;
mod physical_device;

pub use new::InstanceCreateError;
pub use physical_device::{EnumeratePhysicalDevicesError, PhysicalDevice};

/// The instance of this library for the application to interact with
pub struct Instance {
    /// The Vulkan instance
    instance: Arc<vk::Instance>,

    /// The Vulkan debug messenger
    #[allow(unused)]
    debug_messenger: Option<DebugUtilsMessenger>,
}

impl Instance {
    /// Gets the underlying Vulkan instance
    pub(crate) fn inner(&self) -> &Arc<vk::Instance> {
        &self.instance
    }
}
