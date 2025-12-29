use crate::GraphicsVersion;
use vulkan::vk_make_api_version;

impl GraphicsVersion {
    /// Create a new [`GraphicsVersion`]
    pub const fn new(variant: u8, major: u8, minor: u16, patch: u16) -> GraphicsVersion {
        assert!(variant < 8, "`variant` must be less than 8");
        assert!(major < 128, "`major` must be less than 128");
        assert!(minor < 1024, "`minor` must be less than 1024");
        assert!(patch < 4096, "`patch` must be less than 4096");

        let version = vk_make_api_version!(variant, major, minor, patch);
        unsafe { GraphicsVersion::new_raw(version) }
    }

    /// Create a new [`GraphicsVersion`] from a raw `version` value
    pub(crate) const unsafe fn new_raw(version: u32) -> GraphicsVersion {
        GraphicsVersion { version }
    }
}
