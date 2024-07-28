use std::{
    ffi::CStr,
    fmt::{Display, Formatter},
};
use vulkan::VkResult;

/// An error ocurred while creating an object
#[derive(Debug, Clone, Copy)]
pub enum CreateError {
    /// Creating the object itself failed
    CreateFailed(VkResult),

    /// A function the object requires is missing
    MissingFunction(&'static CStr),
}

impl std::error::Error for CreateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CreateError::CreateFailed(error) => Some(error),
            CreateError::MissingFunction(_) => None,
        }
    }
}

impl Display for CreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateError::CreateFailed(error) => error.fmt(f),
            CreateError::MissingFunction(function) => write!(
                f,
                "Failed to get a required Vulkan function \"{}\"",
                function.to_string_lossy().trim()
            ),
        }
    }
}

impl From<VkResult> for CreateError {
    fn from(error: VkResult) -> Self {
        CreateError::CreateFailed(error)
    }
}

impl From<&'static CStr> for CreateError {
    fn from(name: &'static CStr) -> Self {
        CreateError::MissingFunction(name)
    }
}
