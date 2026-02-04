use crate::gpu::VulkanExtension;

impl std::fmt::Display for VulkanExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.name, self.version)
    }
}
