use crate::{
    Result,
    gpu::{VulkanInstance, VulkanInstanceBuilder, instance::VulkanInstanceInner},
};
use std::sync::Arc;

impl<'a> VulkanInstanceBuilder<'a> {
    /// Create a new [`VulkanInstance`] with the provided settings
    pub fn create(&self) -> Result<VulkanInstance> {
        Ok(VulkanInstance {
            inner: Arc::new(VulkanInstanceInner::new(
                &self.context,
                self.api_version,
                self.application.as_ref().map(|(s, v)| (s.as_ref(), *v)),
                self.engine.as_ref().map(|(s, v)| (s.as_ref(), *v)),
                &self.extensions,
                &self.layers,
            )?),
        })
    }
}
