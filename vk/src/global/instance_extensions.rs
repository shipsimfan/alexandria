use super::Global;
use crate::InstanceExtension;
use std::ptr::{null, null_mut};
use vulkan::{VkExtensionProperties, VkResult};

impl Global {
    /// Gets the extensions available for this device
    pub fn instance_extensions(&self) -> Result<Vec<InstanceExtension>, VkResult> {
        let extension_count = self.vk_enumerate_instance_extension_properties(null_mut(), 0)?;
        let mut extensions = Vec::with_capacity(extension_count);

        let extension_count = self
            .vk_enumerate_instance_extension_properties(extensions.as_mut_ptr(), extension_count)?;
        unsafe { extensions.set_len(extension_count) };

        Ok(extensions
            .into_iter()
            .filter_map(|extension| InstanceExtension::from_properties(&extension))
            .collect())
    }

    /// Calls `vkEnumerateInstanceExtensionProperties`
    fn vk_enumerate_instance_extension_properties(
        &self,
        extensions: *mut VkExtensionProperties,
        extension_count: usize,
    ) -> Result<usize, VkResult> {
        let mut extension_count = extension_count as u32;
        self.f()
            .enumerate_instance_extension_properties(null(), &mut extension_count, extensions)
            .map(|_| extension_count as usize)
    }
}
