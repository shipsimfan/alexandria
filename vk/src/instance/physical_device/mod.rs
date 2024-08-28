use super::Instance;
use crate::functions::PhysicalDeviceFunctions;
use std::sync::Arc;
use vulkan::VkPhysicalDevice;

mod device_extensions;
mod is_surface_supported;
mod new;
mod properties;
mod queue_families;
mod surface_capabilities;
mod surface_formats;
mod surface_present_modes;

pub use properties::PhysicalDeviceProperties;

/// A physical device with Vulkan capabilities
pub struct PhysicalDevice {
    /// The handle to the physical device
    handle: VkPhysicalDevice,

    /// The instance the physical device belongs to
    instance: Arc<Instance>,
}

impl PhysicalDevice {
    /// Gets the underlying handle to the physical device
    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }

    /// Gets the instance that created this physical device
    pub(crate) fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    /// Gets the functions for physical devices
    pub(crate) fn f(&self) -> &PhysicalDeviceFunctions {
        self.instance.f().pd()
    }
}

unsafe impl Send for PhysicalDevice {}
unsafe impl Sync for PhysicalDevice {}
