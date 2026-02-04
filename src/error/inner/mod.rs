use std::borrow::Cow;

mod as_error;
mod display;
mod from;

/// A source of an error containing more details
#[derive(Debug)]
pub enum ErrorInner {
    /// An error from Vulkan
    Vulkan(vulkan::VkResult),

    /// An error from Windows
    #[cfg(target_os = "windows")]
    Win32(win32::Error),

    /// Some other error source
    Other(Cow<'static, str>),
}
