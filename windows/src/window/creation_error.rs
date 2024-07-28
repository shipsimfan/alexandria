use std::fmt::{Display, Formatter};

/// The source of the error
#[derive(Debug, Clone)]
enum WindowCreationErrorSource {
    /// Creating the window class failed
    ClassCreationFailed,

    /// Creating the window itself failed
    WindowCreationFailed,
}

/// An error that occured while creating a window
#[derive(Debug, Clone)]
pub struct WindowCreationError {
    /// The source of the error
    source: WindowCreationErrorSource,

    /// The error that caused creation to fail
    error: win32::Error,
}

impl WindowCreationError {
    /// Create a new [`WindowCreationError`] from the result of window class creation
    pub(super) fn class_creation(error: win32::Error) -> Self {
        WindowCreationError {
            source: WindowCreationErrorSource::ClassCreationFailed,
            error,
        }
    }

    /// Create a new [`WindowCreationError`] from the result of window creation
    pub(super) fn window_creation(error: win32::Error) -> Self {
        WindowCreationError {
            source: WindowCreationErrorSource::WindowCreationFailed,
            error,
        }
    }
}

impl std::error::Error for WindowCreationError {}

impl Display for WindowCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.source, self.error)
    }
}

impl Display for WindowCreationErrorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            WindowCreationErrorSource::ClassCreationFailed => "Failed to create the window class",
            WindowCreationErrorSource::WindowCreationFailed => "Failed to create the window",
        })
    }
}
