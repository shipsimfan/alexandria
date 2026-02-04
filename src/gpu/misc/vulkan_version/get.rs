use crate::gpu::VulkanVersion;
use vulkan::{
    vk_api_version_major, vk_api_version_minor, vk_api_version_patch, vk_api_version_variant,
};

impl VulkanVersion {
    /// Get the variant of Vulkan being used
    pub const fn variant(&self) -> u8 {
        vk_api_version_variant!(self.version) as u8
    }

    /// Get the major version of Vulkan being used
    pub const fn major(&self) -> u8 {
        vk_api_version_major!(self.version) as u8
    }

    /// Get the minor version of Vulkan being used
    pub const fn minor(&self) -> u16 {
        vk_api_version_minor!(self.version) as u16
    }

    /// Get the patch version of Vulkan being used
    pub const fn patch(&self) -> u16 {
        vk_api_version_patch!(self.version) as u16
    }

    /// Get the underlying raw Vulkan version
    pub(crate) const fn as_vk(&self) -> u32 {
        self.version
    }
}
