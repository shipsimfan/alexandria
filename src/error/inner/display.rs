use crate::ErrorInner;

impl std::fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorInner::Vulkan(vulkan) => vulkan.fmt(f),
            #[cfg(target_os = "windows")]
            ErrorInner::Win32(error) => error.fmt(f),
            ErrorInner::Other(error) => error.fmt(f),
        }
    }
}
