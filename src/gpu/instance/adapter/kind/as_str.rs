use crate::gpu::VulkanAdapterKind;

impl VulkanAdapterKind {
    /// Get a string representing this adapter kind
    pub fn as_str(&self) -> &'static str {
        match self {
            VulkanAdapterKind::Discrete => "discrete gpu",
            VulkanAdapterKind::Integrated => "integrated gpu",
            VulkanAdapterKind::Virtual => "virtual gpu",
            VulkanAdapterKind::Cpu => "cpu",
            VulkanAdapterKind::Other => "other",
        }
    }
}
