use crate::gpu::{VulkanLayer, VulkanVersion};
use std::{borrow::Cow, ffi::CStr};
use vulkan::VkLayerProperties;

impl VulkanLayer {
    /// Create a new [`VulkanLayer`]
    pub(crate) fn new(layer: &VkLayerProperties) -> VulkanLayer {
        let name_c = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
        let name = match name_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };

        let description_c = unsafe { CStr::from_ptr(layer.description.as_ptr()) };
        let description = match description_c.to_string_lossy() {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        };

        let spec_version = unsafe { VulkanVersion::new_raw(layer.spec_version) };
        let version = unsafe { VulkanVersion::new_raw(layer.implementation_version) };

        VulkanLayer {
            name,
            description,
            spec_version,
            version,
        }
    }
}
