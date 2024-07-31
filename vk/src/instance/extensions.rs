use std::{
    ffi::CStr,
    fmt::{Debug, Display, Formatter},
};
use util::i8_slice_to_cstr;
use vulkan::{
    VkExtensionProperties, VK_EXT_DEBUG_UTILS_EXTENSION_NAME, VK_KHR_SURFACE_EXTENSION_NAME,
    VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
};

/// An extension an instance can use
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstanceExtension {
    /// VK_KHR_surface
    Surface,

    /// VK_EXT_debug_utils
    DebugUtils,

    /// VK_KHR_win32_surface
    Win32Surface,
}

impl InstanceExtension {
    /// Gets an [`InstanceExtension`] from `properties`
    pub(crate) fn from_properties(properties: &VkExtensionProperties) -> Option<Self> {
        match i8_slice_to_cstr(&properties.extension_name) {
            s if s == VK_KHR_SURFACE_EXTENSION_NAME => Some(InstanceExtension::Surface),
            s if s == VK_EXT_DEBUG_UTILS_EXTENSION_NAME => Some(InstanceExtension::DebugUtils),
            s if s == VK_KHR_WIN32_SURFACE_EXTENSION_NAME => Some(InstanceExtension::Win32Surface),
            _ => None,
        }
    }

    /// Gets the extension name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            InstanceExtension::Surface => "VK_KHR_surface",
            InstanceExtension::DebugUtils => "VK_EXT_debug_utils",
            InstanceExtension::Win32Surface => "VK_KHR_win32_surface",
        }
    }

    /// Gets the extension name as a [`CStr`]
    pub(super) fn as_cstr(&self) -> &'static CStr {
        match self {
            InstanceExtension::Surface => VK_KHR_SURFACE_EXTENSION_NAME,
            InstanceExtension::DebugUtils => VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
            InstanceExtension::Win32Surface => VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
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
