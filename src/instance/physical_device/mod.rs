use crate::PhysicalDeviceType;
use check_extension_support::check_extension_support;
use queue_families::QueueFamilies;
use vk::{
    DeviceExtension, PhysicalDeviceProperties, VkPresentModeKHR, VkSurfaceCapabilitiesKHR,
    VkSurfaceFormatKHR, Window,
};

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

    /// The capabilities this device supports on the surface
    capabilities: VkSurfaceCapabilitiesKHR,

    /// The formats this device supports on the surface
    formats: Vec<VkSurfaceFormatKHR>,

    /// The present modes this device supports on the surface
    present_modes: Vec<VkPresentModeKHR>,
}

impl PhysicalDevice {
    /// Creates a new [`PhysicalDevice`]
    pub(self) fn new(physical_device: vk::PhysicalDevice, window: &Window) -> Option<Self> {
        let queue_families = QueueFamilies::new(&physical_device, window.surface())?;
        let extensions = check_extension_support(&physical_device)?;

        let properties = physical_device.properties();

        let capabilities = physical_device
            .surface_capabilities(window.surface())
            .ok()?;
        let formats = physical_device.surface_formats(window.surface()).ok()?;
        let present_modes = physical_device
            .surface_present_modes(window.surface())
            .ok()?;
        if formats.len() == 0 || present_modes.len() == 0 {
            return None;
        }

        Some(PhysicalDevice {
            physical_device,
            queue_families,
            properties,
            extensions,
            capabilities,
            formats,
            present_modes,
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
