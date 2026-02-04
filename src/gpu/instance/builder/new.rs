use crate::{
    AlexandriaContext,
    gpu::{VulkanInstanceBuilder, VulkanVersion},
};

impl<'a> VulkanInstanceBuilder<'a> {
    /// Create a new [`VulkanInstanceBuilder`]
    pub(in crate::gpu) fn new(
        context: AlexandriaContext,
        api_version: VulkanVersion,
    ) -> VulkanInstanceBuilder<'a> {
        VulkanInstanceBuilder {
            context,
            api_version,
            application: None,
            engine: None,
            extensions: Vec::new(),
            layers: Vec::new(),
        }
    }
}
