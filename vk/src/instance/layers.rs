use std::{
    ffi::CStr,
    fmt::{Debug, Display, Formatter},
};
use util::i8_slice_to_cstr;
use vulkan::VkLayerProperties;

/// A layer an instance can use
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstanceLayer {
    /// VK_LAYER_KHRONOS_validation
    Validation,
}

const VK_LAYER_KHRONOS_VALIDATION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") };

impl InstanceLayer {
    /// Gets an [`InstanceLayer`] from `properties`
    pub(crate) fn from_properties(properties: &VkLayerProperties) -> Option<Self> {
        match i8_slice_to_cstr(&properties.layer_name) {
            s if s == VK_LAYER_KHRONOS_VALIDATION => Some(InstanceLayer::Validation),
            _ => None,
        }
    }

    /// Gets the layer name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            InstanceLayer::Validation => "VK_LAYER_KHRONOS_validation",
        }
    }

    /// Gets the layer name as a [`CStr`]
    pub(super) fn as_cstr(&self) -> &'static CStr {
        match self {
            InstanceLayer::Validation => VK_LAYER_KHRONOS_VALIDATION,
        }
    }
}

impl Display for InstanceLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for InstanceLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
