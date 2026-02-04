use crate::gpu::VulkanAdapterKind;

impl std::fmt::Display for VulkanAdapterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
