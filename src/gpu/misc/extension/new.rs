use crate::gpu::{VulkanExtension, VulkanVersion};
use std::{borrow::Cow, ffi::CStr};
use vulkan::VkExtensionProperties;

impl VulkanExtension {
    /// Create a new [`VulkanExtension`]
    pub(crate) fn new(extension: &VkExtensionProperties) -> VulkanExtension {
        let name_c = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };
        let name = match name_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };
        let version = unsafe { VulkanVersion::new_raw(extension.spec_version) };

        VulkanExtension { name, version }
    }
}
