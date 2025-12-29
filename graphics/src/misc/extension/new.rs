use crate::{GraphicsExtension, GraphicsVersion};
use std::{borrow::Cow, ffi::CStr};
use vulkan::VkExtensionProperties;

impl GraphicsExtension {
    /// Create a new [`GraphicsExtension`]
    pub(crate) fn new(extension: &VkExtensionProperties) -> GraphicsExtension {
        let name_c = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };
        let name = match name_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };
        let version = unsafe { GraphicsVersion::new_raw(extension.spec_version) };

        GraphicsExtension { name, version }
    }
}
