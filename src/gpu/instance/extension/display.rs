use crate::gpu::VulkanInstanceExtension;

impl std::fmt::Display for VulkanInstanceExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulkanInstanceExtension::DebugUtils => write!(f, "debug utils"),
            VulkanInstanceExtension::Surface => write!(f, "surface"),
            #[cfg(target_os = "windows")]
            VulkanInstanceExtension::Win32Surface => write!(f, "win32 surface"),
            #[cfg(target_os = "linux")]
            VulkanInstanceExtension::WaylandSurface => write!(f, "wayland surface"),
        }
    }
}
