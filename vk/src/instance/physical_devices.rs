use crate::{Instance, PhysicalDevice};
use std::{ptr::null_mut, sync::Arc};
use vulkan::VkResult;

impl Instance {
    /// Enumerates the physical devices accessible to a Vulkan instance
    pub fn physical_devices(self: &Arc<Self>) -> Result<Vec<PhysicalDevice>, VkResult> {
        let mut count = 0;
        self.f()
            .enumerate_physical_devices(self.handle, &mut count, null_mut())?;

        let mut physical_devices = Vec::with_capacity(count as usize);
        self.f().enumerate_physical_devices(
            self.handle,
            &mut count,
            physical_devices.as_mut_ptr(),
        )?;
        unsafe { physical_devices.set_len(count as usize) };

        Ok(physical_devices
            .into_iter()
            .map(|physical_device| PhysicalDevice::new(physical_device, self.clone()))
            .collect())
    }
}
