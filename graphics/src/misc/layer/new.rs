use crate::{GraphicsLayer, GraphicsVersion};
use std::{borrow::Cow, ffi::CStr};
use vulkan::VkLayerProperties;

impl GraphicsLayer {
    /// Create a new [`GraphicsLayer`]
    pub(crate) fn new(layer: &VkLayerProperties) -> GraphicsLayer {
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

        let spec_version = unsafe { GraphicsVersion::new_raw(layer.spec_version) };
        let version = unsafe { GraphicsVersion::new_raw(layer.implementation_version) };

        GraphicsLayer {
            name,
            description,
            spec_version,
            version,
        }
    }
}
