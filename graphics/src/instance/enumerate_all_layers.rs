use crate::{GraphicsError, GraphicsInstance, GraphicsLayer, Result};
use std::ptr::null_mut;
use vulkan::{
    VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES, VkEnumerateInstanceLayerProperties, try_vulkan,
    vkGetInstanceProcAddr,
};

impl GraphicsInstance {
    /// Enumerate all Vulkan layers supported on this system, even if Alexandria doesn't support
    /// them
    pub fn enumerate_all_layers() -> Result<Vec<GraphicsLayer>> {
        // Get the "vkEnumerateInstanceLayerProperties" function
        let enumerate_instance_layer_properties: VkEnumerateInstanceLayerProperties = unsafe {
            std::mem::transmute(
                vkGetInstanceProcAddr(null_mut(), VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES.as_ptr())
                    .ok_or(GraphicsError::new(
                        "unable to find \"vkEnumerateInstanceLayerProperties\"",
                    ))?,
            )
        };

        // Get the number of layers supported
        let mut layer_count = 0;
        try_vulkan!(enumerate_instance_layer_properties(
            &mut layer_count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get instance layer count", vk))?;
        if layer_count == 0 {
            return Ok(Vec::new());
        }

        // Get the layers
        let mut layers = Vec::with_capacity(layer_count as usize);
        try_vulkan!(enumerate_instance_layer_properties(
            &mut layer_count,
            layers.as_mut_ptr()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get instance layers", vk))?;

        unsafe { layers.set_len(layer_count as usize) };

        // Convert them into `GraphicsExtension`s
        Ok(layers
            .into_iter()
            .map(|layer| GraphicsLayer::new(&layer))
            .collect())
    }
}
