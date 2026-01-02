use crate::{GraphicsInstance, GraphicsVersion};
use alexandria_util::{MemorySize, UUID};
use vulkan::VkPhysicalDevice;

mod functions;
mod kind;

mod get;
mod new;

pub use kind::GraphicsAdapterKind;

pub(in crate::instance) use functions::GraphicsAdapterFunctions;

/// A physical device which can be used for Vulkan
pub struct GraphicsAdapter<'instance> {
    /// The underlying Vulkan physical device
    adapter: VkPhysicalDevice,

    /// The version of Vulkan supported by the device
    api_version: GraphicsVersion,

    /// The version of the graphics driver this device uses
    driver_version: GraphicsVersion,

    /// The kind of graphics adapter this is
    kind: GraphicsAdapterKind,

    /// The name of the adapter
    name: String,

    /// The UUID of the adapter
    uuid: UUID,

    /// The amount of video memory on the adapter
    vram: MemorySize,

    /// The instance this adapter comes from
    _instance: &'instance GraphicsInstance,
}

unsafe impl<'instance> Send for GraphicsAdapter<'instance> {}
unsafe impl<'instance> Sync for GraphicsAdapter<'instance> {}
