use crate::gpu::VulkanLayer;

impl std::fmt::Display for VulkanLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.name, self.spec_version)
    }
}
