use super::PhysicalDevice;
use crate::{Instance, Window};
use std::fmt::Display;
use util::Error;
use vk::VkResult;

/// An error occurred while getting the physical devices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnumeratePhysicalDevicesError(VkResult);

impl Instance {
    /// Gets the physical devices compatible with `window`
    pub fn physical_devices(
        &self,
        window: &Window,
    ) -> Result<Vec<PhysicalDevice>, EnumeratePhysicalDevicesError> {
        self.instance
            .physical_devices()
            .map(|physical_devices| {
                physical_devices
                    .into_iter()
                    .filter_map(|physical_device| {
                        PhysicalDevice::new(physical_device, window.inner())
                    })
                    .collect()
            })
            .map_err(EnumeratePhysicalDevicesError)
    }
}

impl Error for EnumeratePhysicalDevicesError {
    fn title(&self) -> &'static str {
        "Enumerate Physical Devices Error"
    }
}

impl std::error::Error for EnumeratePhysicalDevicesError {}

impl Display for EnumeratePhysicalDevicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to enumerate the physical devices - {}", self.0)
    }
}
