use std::{
    ffi::CStr,
    fmt::{Debug, Display, Formatter},
};
use util::i8_slice_to_cstr;
use vulkan::{VkExtensionProperties, VK_EXT_DEBUG_UTILS_EXTENSION_NAME};

/// An extension an instance can use
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstanceExtension {
    /// VK_EXT_debug_utils
    DebugUtils,
}

impl InstanceExtension {
    /// Required Vulkan instance extensions for Windows
    #[cfg(target_os = "windows")]
    pub fn os() -> &'static [InstanceExtension] {
        &[]
    }

    /// Gets an [`InstanceExtension`] from `properties`
    pub(crate) fn from_properties(properties: &VkExtensionProperties) -> Option<Self> {
        match i8_slice_to_cstr(&properties.extension_name) {
            s if s == VK_EXT_DEBUG_UTILS_EXTENSION_NAME => Some(InstanceExtension::DebugUtils),
            _ => None,
        }
    }

    /// Gets the extension name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            InstanceExtension::DebugUtils => "VK_EXT_debug_utils",
        }
    }

    /// Gets the extension name as a [`CStr`]
    pub(super) fn as_cstr(&self) -> &'static CStr {
        match self {
            InstanceExtension::DebugUtils => VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
        }
    }
}

impl Display for InstanceExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for InstanceExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
