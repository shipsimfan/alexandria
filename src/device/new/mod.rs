use super::Device;
use crate::PhysicalDevice;

mod error;

pub use error::DeviceCreateError;

impl Device {
    /// Creates a new [`Device`]
    pub fn new(physical_device: &PhysicalDevice) -> Result<Self, DeviceCreateError> {
        let device = vk::Device::new(
            physical_device.handle(),
            physical_device.queue_families().to_vec(),
        )?;

        Ok(Device { device })
    }
}
