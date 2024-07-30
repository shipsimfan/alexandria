use super::PhysicalDevice;
use std::ptr::null_mut;
use vulkan::VkQueueFamilyProperties;

impl PhysicalDevice {
    /// Gets the queue families available for the device
    pub fn queue_families(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0;
        self.f()
            .get_physical_device_queue_family_properties(self.handle, &mut count, null_mut());

        let mut queue_families = Vec::with_capacity(count as usize);
        self.f().get_physical_device_queue_family_properties(
            self.handle,
            &mut count,
            queue_families.as_mut_ptr(),
        );
        unsafe { queue_families.set_len(count as usize) };

        queue_families
    }
}
