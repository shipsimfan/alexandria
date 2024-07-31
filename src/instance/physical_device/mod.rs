use crate::PhysicalDeviceType;
use queue_families::QueueFamilies;
use vk::{PhysicalDeviceProperties, Window};

mod get;
mod queue_families;

pub use get::EnumeratePhysicalDevicesError;

/// A physical device which can be used to
pub struct PhysicalDevice {
    /// Handle to the physical device
    physical_device: vk::PhysicalDevice,

    /// The queue families available on the device
    queue_families: QueueFamilies,

    /// The properties describing this physical device
    properties: PhysicalDeviceProperties,
}

impl PhysicalDevice {
    /// Creates a new [`PhysicalDevice`]
    pub(self) fn new(physical_device: vk::PhysicalDevice, window: &Window) -> Option<Self> {
        let queue_families = QueueFamilies::new(&physical_device)?;
        let properties = physical_device.properties();

        Some(PhysicalDevice {
            physical_device,
            queue_families,
            properties,
        })
    }

    /// Gets the name of the vendor provided name of the physical device
    pub fn name(&self) -> &str {
        &self.properties.name
    }

    /// Gets the hardware type of the physical device
    pub fn r#type(&self) -> PhysicalDeviceType {
        self.properties.r#type
    }

    /// Gets the handle to the underlying physical device
    pub(crate) fn handle(&self) -> &vk::PhysicalDevice {
        &self.physical_device
    }

    /// Gets the available queue families for this device
    pub(crate) fn queue_families(&self) -> &QueueFamilies {
        &self.queue_families
    }
}
