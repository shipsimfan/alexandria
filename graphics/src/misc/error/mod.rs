use std::borrow::Cow;
use vulkan::VkResult;

mod display;
mod error;
mod new;

/// A result of a graphics API call
pub type Result<T> = std::result::Result<T, GraphicsError>;

/// An error that can occur while running a value in the graphics API
#[derive(Debug)]
pub struct GraphicsError {
    /// The message describing the circumstances of the error, or the error itself
    message: Cow<'static, str>,

    /// The error from Vulkan, if it originated from there
    vk: Option<VkResult>,
}
