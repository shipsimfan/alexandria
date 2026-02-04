use crate::gpu::VulkanLayer;

impl PartialEq for VulkanLayer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.spec_version == other.spec_version
    }
}

impl Eq for VulkanLayer {}
