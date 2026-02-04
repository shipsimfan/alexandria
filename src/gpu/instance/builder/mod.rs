use crate::{
    AlexandriaContext,
    gpu::{VulkanInstanceExtension, VulkanVersion},
};
use std::borrow::Cow;

mod create;
mod get;
mod new;
mod set;

/// A builder for [`VulkanInstance`](crate::VulkanInstance)s
pub struct VulkanInstanceBuilder<'a> {
    /// The context to use to build the instance
    context: AlexandriaContext,

    /// The requested version of Vulkan to use
    api_version: VulkanVersion,

    /// The name and version of the application being run
    application: Option<(Cow<'a, str>, VulkanVersion)>,

    /// The name and version of the engine being run
    engine: Option<(Cow<'a, str>, VulkanVersion)>,

    /// The requested extensions to enable for the instance
    extensions: Vec<VulkanInstanceExtension>,

    /// The requested layers to enable for the instance
    layers: Vec<Cow<'a, str>>,
}
