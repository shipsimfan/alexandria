use crate::{
    Result,
    gpu::{VulkanAdapter, VulkanInstance},
};

impl VulkanInstance {
    /// Enumerate all the [`VulkanAdapter`]s on the system
    pub fn enumerate_adapters<'instance>(&'instance self) -> Result<Vec<VulkanAdapter<'instance>>> {
        self.inner.enumerate_adapters(self)
    }
}
