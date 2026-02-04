use crate::{
    Error, Result,
    gpu::{VulkanAdapter, VulkanInstance, instance::VulkanInstanceInner},
};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl VulkanInstanceInner {
    /// Enumerate all the [`VulkanAdapter`]s on the system
    pub(in crate::gpu::instance) fn enumerate_adapters<'instance>(
        &self,
        instance: &'instance VulkanInstance,
    ) -> Result<Vec<VulkanAdapter<'instance>>> {
        // Get the number of adapters
        let mut adapter_count = 0;
        try_vulkan!((self.functions.enumerate_physical_devices)(
            self.handle,
            &mut adapter_count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get the number of adapters", vk))?;
        if adapter_count == 0 {
            return Ok(Vec::new());
        }

        // Get the adapter handles
        let mut adapters = Vec::with_capacity(adapter_count as _);
        try_vulkan!((self.functions.enumerate_physical_devices)(
            self.handle,
            &mut adapter_count,
            adapters.as_mut_ptr(),
        ))
        .map_err(|vk| Error::new_with("unable to enumerate adapters", vk))?;
        unsafe { adapters.set_len(adapter_count as usize) };

        // Convert the handles into `VulkanAdapter`s
        let mut adapters: Vec<_> = adapters
            .into_iter()
            .map(|handle| VulkanAdapter::new(instance, handle))
            .collect();
        adapters.sort_by(|a, b| b.cmp(a));
        Ok(adapters)
    }
}
