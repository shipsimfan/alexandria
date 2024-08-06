use std::{
    ffi::CStr,
    fmt::{Debug, Display, Formatter},
};
use util::i8_slice_to_cstr;
use vulkan::{VkExtensionProperties, VK_KHR_SWAPCHAIN_EXTENSION_NAME};

/// An extension an instance can use
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeviceExtension {
    /// VK_KHR_Swapchain
    Swapchain,
}

impl DeviceExtension {
    /// Gets an [`InstanceExtension`] from `properties`
    pub(crate) fn from_properties(properties: &VkExtensionProperties) -> Option<Self> {
        match i8_slice_to_cstr(&properties.extension_name) {
            s if s == VK_KHR_SWAPCHAIN_EXTENSION_NAME => Some(DeviceExtension::Swapchain),
            _ => None,
        }
    }

    /// Gets the extension name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceExtension::Swapchain => "VK_KHR_swapchain",
        }
    }

    /// Gets the extension name as a [`CStr`]
    pub(super) fn as_cstr(&self) -> &'static CStr {
        match self {
            DeviceExtension::Swapchain => VK_KHR_SWAPCHAIN_EXTENSION_NAME,
        }
    }
}

impl Display for DeviceExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for DeviceExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
