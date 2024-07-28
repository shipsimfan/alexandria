use std::fmt::{Display, Formatter};
use util::Error;
use vk::{CreateError, InstanceExtension, InstanceLayer, VkResult};

/// An error occurred while creating the graphics context
#[derive(Debug)]
pub enum InstanceCreateError {
    /// Failed to load the Vulkan global functions
    GlobalFunctionMissing(CreateError),

    /// Failed to enumerate the Vulkan instance layers
    EnumerateLayersFailed(VkResult),

    /// Missing required Vulkan instance layers
    MissingLayers(Vec<InstanceLayer>),

    /// Failed to enumerate the Vulkan instance extensions
    EnumerateExtensionsFailed(VkResult),

    /// Missing required Vulkan instance extensions
    MissingExtensions(Vec<InstanceExtension>),

    /// Failed to create the Vulkan instance
    CreateInstanceFailed(CreateError),

    /// Failed to create the Vulkan debug messenger
    CreateDebugMessengerFailed(VkResult),

    /// Failed to enumerate the physical devices
    EnumeratePhysicalDevicesFailed(VkResult),
}

impl Error for InstanceCreateError {
    fn title(&self) -> &'static str {
        "Graphics Instance Creation Error"
    }
}

impl std::error::Error for InstanceCreateError {}

impl Display for InstanceCreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceCreateError::GlobalFunctionMissing(error) => error.fmt(f),

            InstanceCreateError::EnumerateLayersFailed(error) => {
                write!(
                    f,
                    "Failed to enumerate the Vulkan instance layers - {}",
                    error
                )
            }
            InstanceCreateError::MissingLayers(layers) => {
                write!(f, "Missing required Vulkan instance layers: {:?}", layers)
            }
            InstanceCreateError::EnumerateExtensionsFailed(error) => {
                write!(
                    f,
                    "Failed to enumerate the Vulkan instance extensions - {}",
                    error
                )
            }
            InstanceCreateError::MissingExtensions(extensions) => {
                write!(
                    f,
                    "Missing required Vulkan instance extensions: {:?}",
                    extensions
                )
            }
            InstanceCreateError::CreateInstanceFailed(error) => {
                write!(f, "Failed to create the Vulkan instance - {}", error)
            }
            InstanceCreateError::CreateDebugMessengerFailed(error) => {
                write!(f, "Failed to create the Vulkan debug messenger - {}", error)
            }
            InstanceCreateError::EnumeratePhysicalDevicesFailed(error) => {
                write!(f, "Failed to enumerate the physicals devices - {}", error)
            }
        }
    }
}
