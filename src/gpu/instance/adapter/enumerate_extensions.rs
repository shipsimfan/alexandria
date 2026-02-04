use crate::{
    Result,
    gpu::{VulkanAdapter, VulkanDeviceExtension},
};

impl<'instance> VulkanAdapter<'instance> {
    /// Enumerate all Alexandria supported Vulkan extensions supported on this adapter
    pub fn enumerate_extensions(&self) -> Result<Vec<VulkanDeviceExtension>> {
        Ok(self
            .enumerate_all_extensions()?
            .into_iter()
            .filter_map(|extension| VulkanDeviceExtension::from_str(extension.name()))
            .collect())
    }
}
