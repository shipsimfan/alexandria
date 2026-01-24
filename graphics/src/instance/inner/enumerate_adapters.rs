use crate::{
    GraphicsAdapter, GraphicsError, GraphicsInstance, Result, instance::GraphicsInstanceInner,
};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl GraphicsInstanceInner {
    /// Enumerate all the [`GraphicsAdapter`]s on the system
    pub(in crate::instance) fn enumerate_adapters<'instance>(
        &self,
        instance: &'instance GraphicsInstance,
    ) -> Result<Vec<GraphicsAdapter<'instance>>> {
        // Get the number of adapters
        let mut adapter_count = 0;
        try_vulkan!((self.functions.enumerate_physical_devices)(
            self.handle,
            &mut adapter_count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get the number of adapters", vk))?;
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
        .map_err(|vk| GraphicsError::new_vk("unable to enumerate adapters", vk))?;
        unsafe { adapters.set_len(adapter_count as usize) };

        // Convert the handles into `GraphicsAdapter`s
        let mut adapters: Vec<_> = adapters
            .into_iter()
            .map(|handle| GraphicsAdapter::new(instance, handle))
            .collect();
        adapters.sort_by(|a, b| b.cmp(a));
        Ok(adapters)
    }
}
