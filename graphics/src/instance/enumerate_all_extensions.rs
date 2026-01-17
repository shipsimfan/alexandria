use crate::{
    GraphicsError, GraphicsExtension, GraphicsInstance, GraphicsLayer, Result,
    util::load_global_function,
};
use std::{ffi::CString, ptr::null_mut};
use vulkan::{
    VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES, VkEnumerateInstanceExtensionProperties, try_vulkan,
};

impl GraphicsInstance {
    /// Enumerate all Vulkan extensions supported on this system, even if Alexandria doesn't
    /// support them
    pub fn enumerate_all_extensions(
        layer: Option<&GraphicsLayer>,
    ) -> Result<Vec<GraphicsExtension>> {
        // Get the "vkEnumerateInstanceExtensionProperties" function
        let enumerate_instance_extension_properties: VkEnumerateInstanceExtensionProperties =
            load_global_function!(VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES)?;

        // Extract layer name
        let layer_name = layer.map(|layer| CString::new(layer.name()).unwrap());
        let layer_name_ptr = layer_name
            .as_ref()
            .map(|str| str.as_ptr())
            .unwrap_or(null_mut());

        // Get the number of extensions supported
        let mut extension_count = 0;
        try_vulkan!(enumerate_instance_extension_properties(
            layer_name_ptr,
            &mut extension_count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get instance extension count", vk))?;
        if extension_count == 0 {
            return Ok(Vec::new());
        }

        // Get the extensions
        let mut extensions = Vec::with_capacity(extension_count as usize);
        try_vulkan!(enumerate_instance_extension_properties(
            layer_name_ptr,
            &mut extension_count,
            extensions.as_mut_ptr()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get instance extensions", vk))?;

        unsafe { extensions.set_len(extension_count as usize) };

        // Convert them into `GraphicsExtension`s
        Ok(extensions
            .into_iter()
            .map(|extension| GraphicsExtension::new(&extension))
            .collect())
    }
}
