use crate::gpu::VulkanVersion;
use std::{num::ParseIntError, str::FromStr};

impl FromStr for VulkanVersion {
    type Err = VulkanVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');

        let input_variant = parts.next().ok_or(VulkanVersionParseError::InvalidFormat)?;
        let input_major = parts.next().ok_or(VulkanVersionParseError::InvalidFormat)?;
        let input_minor = parts.next().ok_or(VulkanVersionParseError::InvalidFormat)?;
        let input_patch = parts.next();

        if parts.next().is_some() {
            return Err(VulkanVersionParseError::InvalidFormat);
        }

        let (variant, major, minor, patch) = if let Some(patch) = input_patch {
            (Some(input_variant), input_major, input_minor, patch)
        } else {
            (None, input_variant, input_major, input_minor)
        };

        VulkanVersion::from_strs(variant, major, minor, patch)
            .map_err(VulkanVersionParseError::InvalidNumber)
    }
}

/// An error that can occur when parsing a [`VulkanVersion`] from a string
#[derive(Debug)]
pub enum VulkanVersionParseError {
    /// The string is not in a valid format
    InvalidFormat,

    /// One of the version components is not a valid number
    InvalidNumber(ParseIntError),
}

impl std::error::Error for VulkanVersionParseError {}

impl std::fmt::Display for VulkanVersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulkanVersionParseError::InvalidFormat => write!(f, "invalid version format"),
            VulkanVersionParseError::InvalidNumber(e) => e.fmt(f),
        }
    }
}
