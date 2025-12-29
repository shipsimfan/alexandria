use vulkan::VkResult;

use crate::GraphicsError;
use std::borrow::Cow;

impl GraphicsError {
    /// Create a new [`GraphicsError`] containing only a message
    pub(crate) fn new<S: Into<Cow<'static, str>>>(message: S) -> GraphicsError {
        GraphicsError {
            message: message.into(),
            vk: None,
        }
    }

    /// Create a new [`GraphicsError`] from a Vulkan error
    pub(crate) fn new_vk<S: Into<Cow<'static, str>>>(message: S, vk: VkResult) -> GraphicsError {
        GraphicsError {
            message: message.into(),
            vk: Some(vk),
        }
    }
}
