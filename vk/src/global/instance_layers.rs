use crate::{Global, InstanceLayer};
use std::ptr::null_mut;
use vulkan::{VkLayerProperties, VkResult};

impl Global {
    /// Gets the layers available for this device
    pub fn instance_layers(&self) -> Result<Vec<InstanceLayer>, VkResult> {
        let layer_count = self.vk_enumerate_instance_layer_properties(null_mut(), 0)?;
        let mut layers = Vec::with_capacity(layer_count);

        let layer_count =
            self.vk_enumerate_instance_layer_properties(layers.as_mut_ptr(), layer_count)?;
        unsafe { layers.set_len(layer_count) };

        Ok(layers
            .into_iter()
            .filter_map(|layer| InstanceLayer::from_properties(&layer))
            .collect())
    }

    /// Calls `vkEnumerateInstanceLayerProperties`
    fn vk_enumerate_instance_layer_properties(
        &self,
        layers: *mut VkLayerProperties,
        layer_count: usize,
    ) -> Result<usize, VkResult> {
        let mut layer_count = layer_count as u32;
        self.f()
            .enumerate_instance_layer_properties(&mut layer_count, layers)
            .map(|_| layer_count as usize)
    }
}
