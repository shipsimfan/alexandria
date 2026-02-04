use crate::{
    Error, Result,
    gpu::{GpuSubsystem, VulkanLayer},
};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl GpuSubsystem {
    /// Enumerate all Vulkan layers supported on this system
    pub fn layers(&self) -> Result<Vec<VulkanLayer>> {
        // Get the number of layers supported
        let mut layer_count = 0;
        try_vulkan!((self.functions.enumerate_instance_layers)(
            &mut layer_count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get instance layer count", vk))?;
        if layer_count == 0 {
            return Ok(Vec::new());
        }

        // Get the layers
        let mut layers = Vec::with_capacity(layer_count as usize);
        try_vulkan!((self.functions.enumerate_instance_layers)(
            &mut layer_count,
            layers.as_mut_ptr()
        ))
        .map_err(|vk| Error::new_with("unable to get instance layers", vk))?;

        unsafe { layers.set_len(layer_count as usize) };

        // Convert them into `VulkanLayer`s
        Ok(layers
            .into_iter()
            .map(|layer| VulkanLayer::new(&layer))
            .collect())
    }
}
