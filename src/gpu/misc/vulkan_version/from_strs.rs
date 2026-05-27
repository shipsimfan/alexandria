use crate::gpu::VulkanVersion;
use std::num::ParseIntError;

impl VulkanVersion {
    /// Creates a new [`VulkanVersion`] from the given major, minor and patch version strings
    pub fn from_strs(
        variant: Option<&str>,
        major: &str,
        minor: &str,
        patch: &str,
    ) -> Result<VulkanVersion, ParseIntError> {
        let variant = variant.map(|v| v.parse()).transpose()?.unwrap_or(0);
        let major = major.parse()?;
        let minor = minor.parse()?;
        let patch = patch.parse()?;
        Ok(VulkanVersion::new(variant, major, minor, patch))
    }
}
