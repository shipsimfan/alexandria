use crate::ErrorInner;
use std::borrow::Cow;

impl From<vulkan::VkResult> for ErrorInner {
    fn from(error: vulkan::VkResult) -> Self {
        ErrorInner::Vulkan(error)
    }
}

#[cfg(target_os = "windows")]
impl From<win32::Error> for ErrorInner {
    fn from(error: win32::Error) -> Self {
        ErrorInner::Win32(error)
    }
}

impl From<Cow<'static, str>> for ErrorInner {
    fn from(error: Cow<'static, str>) -> Self {
        ErrorInner::Other(error)
    }
}

impl From<&'static String> for ErrorInner {
    fn from(error: &'static String) -> Self {
        ErrorInner::Other(error.into())
    }
}

impl From<&'static str> for ErrorInner {
    fn from(error: &'static str) -> Self {
        ErrorInner::Other(error.into())
    }
}

impl From<String> for ErrorInner {
    fn from(error: String) -> Self {
        ErrorInner::Other(error.into())
    }
}
