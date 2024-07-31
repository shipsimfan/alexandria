use std::fmt::{Display, Formatter};
use util::Error;
use vulkan::VkResult;

/// An error that occured while creating a window
#[derive(Debug, Clone)]
pub enum WindowCreationError {
    /// Creating the window class failed
    ClassCreationFailed(win32::Error),

    /// Creating the window itself failed
    WindowCreationFailed(win32::Error),

    /// Creating the Vulkan surface failed
    SurfaceCreationFailed(VkResult),
}

impl Error for WindowCreationError {
    fn title(&self) -> &'static str {
        "Window Creation Error"
    }
}

impl std::error::Error for WindowCreationError {}

impl Display for WindowCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowCreationError::ClassCreationFailed(error) => {
                write!(f, "Failed to create the window class - {}", error)
            }
            WindowCreationError::WindowCreationFailed(error) => {
                write!(f, "Failed to create the window - {}", error)
            }
            WindowCreationError::SurfaceCreationFailed(error) => {
                write!(f, "Failed to create the Vulkan surface - {}", error)
            }
        }
    }
}

impl From<VkResult> for WindowCreationError {
    fn from(error: VkResult) -> Self {
        WindowCreationError::SurfaceCreationFailed(error)
    }
}
