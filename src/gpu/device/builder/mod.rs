use crate::gpu::{VulkanAdapter, VulkanAdapterFeature, VulkanDeviceExtension};

mod queue_info;

mod create;
mod get;
mod new;
mod set;

pub use queue_info::*;

/// A builder for [`VulkanDevice`](crate::VulkanDevice)s
pub struct VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Features to enable for the device to be created
    features: Vec<&'a mut dyn VulkanAdapterFeature>,

    /// The information describing the queues to be created
    queues: Vec<VulkanQueueCreateInfo<'a>>,

    /// The requested extensions to enable for the device
    extensions: Vec<VulkanDeviceExtension>,

    /// The adapter to create the graphics device with
    adapter: &'adapter VulkanAdapter<'instance>,
}
