use crate::gpu::{VulkanAdapter, VulkanQueueFamilyProperties};
use std::ptr::null_mut;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the queue family properties of the adapter
    pub fn get_queue_family_properties(&self) -> Vec<VulkanQueueFamilyProperties> {
        // Get the number of queue families
        let mut count = 0;
        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_queue_family_properties)(
                self.handle, &mut count, null_mut()
            )
        };

        // Get the queue family properties
        let mut properties: Vec<VulkanQueueFamilyProperties> = Vec::with_capacity(count as usize);
        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_queue_family_properties)(
                self.handle,
                &mut count,
                properties.as_mut_ptr().cast(),
            )
        };
        unsafe { properties.set_len(count as usize) };

        properties
    }
}
