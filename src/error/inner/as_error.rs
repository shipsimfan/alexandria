use crate::ErrorInner;

impl ErrorInner {
    /// Get this value as a [`std::error::Error`]
    pub fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorInner::Vulkan(vulkan) => Some(vulkan),
            #[cfg(target_os = "windows")]
            ErrorInner::Win32(error) => Some(error),
            ErrorInner::Other(_) => None,
        }
    }
}
