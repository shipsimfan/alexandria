use crate::{
    MemorySize, Uuid,
    gpu::{VulkanAdapterKind, VulkanInstance, VulkanVersion},
};
use vulkan::VkPhysicalDevice;

mod features;
mod functions;
mod queue_family_properties;
mod surface_capabilities;

mod device_builder;
mod enumerate_all_extensions;
mod enumerate_extensions;
mod eq;
mod get;
mod get_features;
mod new;
mod ord;
mod supports_surface;
mod surface_present_modes;
mod swapchain_formats;

pub use features::*;
pub use queue_family_properties::*;
pub use surface_capabilities::*;

pub(in crate::gpu::instance) use functions::VulkanAdapterFunctions;

/// A physical device which can be used for Vulkan
pub struct VulkanAdapter<'instance> {
    /// The underlying Vulkan physical device
    handle: VkPhysicalDevice,

    /// The version of Vulkan supported by the device
    api_version: VulkanVersion,

    /// The version of the graphics driver this device uses
    driver_version: VulkanVersion,

    /// The kind of graphics adapter this is
    kind: VulkanAdapterKind,

    /// The name of the adapter
    name: String,

    /// The UUID of the adapter
    uuid: Uuid,

    /// The amount of video memory on the adapter
    vram: MemorySize,

    /// The supported queue families on this device
    queue_families: Vec<VulkanQueueFamilyProperties>,

    /// The instance this adapter comes from
    instance: &'instance VulkanInstance,
}

unsafe impl<'instance> Send for VulkanAdapter<'instance> {}
unsafe impl<'instance> Sync for VulkanAdapter<'instance> {}
