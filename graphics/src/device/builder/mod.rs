use crate::{GraphicsDeviceExtension, GraphicsInstance};
use vulkan::VkDeviceQueueCreateInfo;

mod extended_info;
mod queue_info;

mod get;
mod new;
mod set;

pub(in crate::device) use extended_info::VkGraphicsDeviceExtendedCreateInfo;

pub use extended_info::GraphicsDeviceExtendedCreateInfo;
pub use queue_info::GraphicsQueueCreateInfo;

/// A builder for [`GraphicsDevice`](crate::GraphicsDevice)s
pub struct GraphicsDeviceBuilder<'instance, 'a> {
    /// Extended information that can adjusts the device that will be created
    extended_info: Vec<GraphicsDeviceExtendedCreateInfo>,

    /// The information describing the queues to be created
    queues: Vec<GraphicsQueueCreateInfo<'a>>,

    /// The requested extensions to enable for the device
    extensions: Vec<GraphicsDeviceExtension>,

    /// The instance to create the graphics device with
    instance: &'instance GraphicsInstance,
}
