use crate::{GraphicsAdapter, GraphicsDeviceExtension};

mod extended_info;
mod queue_info;

mod create;
mod get;
mod new;
mod set;

pub use extended_info::*;
pub use queue_info::GraphicsQueueCreateInfo;

/// A builder for [`GraphicsDevice`](crate::GraphicsDevice)s
pub struct GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
    /// Extended information that can adjusts the device that will be created
    extended_info: Vec<GraphicsDeviceExtendedCreateInfo>,

    /// The information describing the queues to be created
    queues: Vec<GraphicsQueueCreateInfo<'a>>,

    /// The requested extensions to enable for the device
    extensions: Vec<GraphicsDeviceExtension>,

    /// The adapter to create the graphics device with
    adapter: &'adapter GraphicsAdapter<'instance>,
}
