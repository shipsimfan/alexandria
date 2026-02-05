use crate::{
    Result,
    gpu::{VulkanInstanceExtension, VulkanLayer, subsystem::GpuSubsystemInner},
};

impl GpuSubsystemInner {
    /// Enumerate all Alexandria supported Vulkan extensions supported on this system
    pub fn enumerate_extensions(
        &self,
        layer: Option<&VulkanLayer>,
    ) -> Result<Vec<VulkanInstanceExtension>> {
        Ok(self
            .all_extensions(layer)?
            .into_iter()
            .filter_map(|extension| VulkanInstanceExtension::from_str(extension.name()))
            .collect())
    }
}
