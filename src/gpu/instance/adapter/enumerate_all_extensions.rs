use crate::{
    Error, Result,
    gpu::{VulkanAdapter, VulkanExtension},
};
use std::ptr::{null, null_mut};
use vulkan::try_vulkan;

impl<'instance> VulkanAdapter<'instance> {
    /// Enumerate all Vulkan extensions supported by this adapter, even if Alexandria doesn't
    /// support them
    pub fn enumerate_all_extensions(&self) -> Result<Vec<VulkanExtension>> {
        // Get the number of extensions supported
        let mut extension_count = 0;
        try_vulkan!((self
            .instance
            .functions
            .adapter
            .enumerate_device_extension_properties)(
            self.handle,
            null(),
            &mut extension_count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get device extension count", vk))?;
        if extension_count == 0 {
            return Ok(Vec::new());
        }

        // Get the extensions
        let mut extensions = Vec::with_capacity(extension_count as usize);
        try_vulkan!((self
            .instance
            .functions
            .adapter
            .enumerate_device_extension_properties)(
            self.handle,
            null(),
            &mut extension_count,
            extensions.as_mut_ptr()
        ))
        .map_err(|vk| Error::new_with("unable to get device extensions", vk))?;

        unsafe { extensions.set_len(extension_count as usize) };

        // Convert them into `VulkanExtension`s
        Ok(extensions
            .into_iter()
            .map(|extension| VulkanExtension::new(&extension))
            .collect())
    }
}
