use super::Instance;
use crate::functions::PhysicalDeviceFunctions;
use std::rc::Rc;
use vulkan::VkPhysicalDevice;

mod new;
mod properties;
mod queue_families;

pub use properties::PhysicalDeviceProperties;

/// A physical device with Vulkan capabilities
pub struct PhysicalDevice {
    /// The handle to the physical device
    handle: VkPhysicalDevice,

    /// The instance the physical device belongs to
    instance: Rc<Instance>,
}

impl PhysicalDevice {
    /// Gets the functions for physical devices
    pub(crate) fn f(&self) -> &PhysicalDeviceFunctions {
        self.instance.f().pd()
    }
}
