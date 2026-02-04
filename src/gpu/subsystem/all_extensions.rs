use crate::{
    Error, Result,
    gpu::{GpuSubsystem, VulkanExtension, VulkanLayer},
};
use std::{ffi::CString, ptr::null_mut};
use vulkan::try_vulkan;

impl GpuSubsystem {
    /// Enumerate all Vulkan extensions supported on this system, even if Alexandria doesn't
    /// support them
    pub fn all_extensions(&self, layer: Option<&VulkanLayer>) -> Result<Vec<VulkanExtension>> {
        // Extract layer name
        let layer_name = layer.map(|layer| CString::new(layer.name()).unwrap());
        let layer_name_ptr = layer_name
            .as_ref()
            .map(|str| str.as_ptr())
            .unwrap_or(null_mut());

        // Get the number of extensions supported
        let mut extension_count = 0;
        try_vulkan!((self.functions.enumerate_instance_extensions)(
            layer_name_ptr,
            &mut extension_count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get instance extension count", vk))?;
        if extension_count == 0 {
            return Ok(Vec::new());
        }

        // Get the extensions
        let mut extensions = Vec::with_capacity(extension_count as usize);
        try_vulkan!((self.functions.enumerate_instance_extensions)(
            layer_name_ptr,
            &mut extension_count,
            extensions.as_mut_ptr()
        ))
        .map_err(|vk| Error::new_with("unable to get instance extensions", vk))?;

        unsafe { extensions.set_len(extension_count as usize) };

        // Convert them into `VulkanExtension`s
        Ok(extensions
            .into_iter()
            .map(|extension| VulkanExtension::new(&extension))
            .collect())
    }
}
