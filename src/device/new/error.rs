use util::Error;
use vk::CreateError;

/// An error that can occur while creating the device
#[derive(Debug)]
pub struct DeviceCreateError(CreateError);

impl Error for DeviceCreateError {
    fn title(&self) -> &'static str {
        "Device Creation Error"
    }
}

impl std::error::Error for DeviceCreateError {}

impl std::fmt::Display for DeviceCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to create the device - {}", self.0)
    }
}

impl From<CreateError> for DeviceCreateError {
    fn from(error: CreateError) -> Self {
        DeviceCreateError(error)
    }
}
