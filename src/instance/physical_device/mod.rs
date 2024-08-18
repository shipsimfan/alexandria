use crate::PhysicalDeviceType;
use check_extension_support::check_extension_support;
use queue_families::QueueFamilies;
use vk::{DeviceExtension, PhysicalDeviceProperties, Window};

mod check_extension_support;
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

    /// The extensions this device will use
    extensions: Vec<DeviceExtension>,
}

impl PhysicalDevice {
    /// Creates a new [`PhysicalDevice`]
    pub(self) fn new(physical_device: vk::PhysicalDevice, window: &Window) -> Option<Self> {
        let queue_families = QueueFamilies::new(&physical_device, window.surface())?;
        let properties = physical_device.properties();
        let extensions = check_extension_support(&physical_device)?;

        Some(PhysicalDevice {
            physical_device,
            queue_families,
            properties,
            extensions,
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

    /// Gets the extensions this device will use
    pub(crate) fn extensions(&self) -> &[DeviceExtension] {
        &self.extensions
    }
}
