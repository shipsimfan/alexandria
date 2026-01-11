use crate::GraphicsInstanceExtension;

impl std::fmt::Display for GraphicsInstanceExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsInstanceExtension::DebugUtils => write!(f, "debug utils"),
            GraphicsInstanceExtension::Surface => write!(f, "surface"),
            #[cfg(target_os = "windows")]
            GraphicsInstanceExtension::Win32Surface => write!(f, "win32 surface"),
        }
    }
}
