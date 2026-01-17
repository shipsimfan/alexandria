use crate::{GraphicsVersion, instance::GraphicsInstanceInner};
use alexandria_util::{MemorySize, UUID};
use vulkan::VkPhysicalDevice;

mod functions;
mod kind;
mod queue_family_info;

mod enumerate_all_extensions;
mod eq;
mod get;
mod new;
mod ord;
mod supports_surface;
mod surface_present_modes;
mod swapchain_formats;

pub use kind::GraphicsAdapterKind;
pub use queue_family_info::GraphicsQueueFamilyInfo;

pub(in crate::instance) use functions::GraphicsAdapterFunctions;

/// A physical device which can be used for Vulkan
pub struct GraphicsAdapter<'instance> {
    /// The underlying Vulkan physical device
    handle: VkPhysicalDevice,

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

    /// The supported queue families on this device
    queue_families: Vec<GraphicsQueueFamilyInfo>,

    /// The instance this adapter comes from
    instance: &'instance GraphicsInstanceInner,
}

unsafe impl<'instance> Send for GraphicsAdapter<'instance> {}
unsafe impl<'instance> Sync for GraphicsAdapter<'instance> {}
