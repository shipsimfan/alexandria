use crate::{
    MemorySize, Uuid,
    gpu::{VulkanInstance, VulkanVersion},
};
use vulkan::VkPhysicalDevice;

mod functions;
mod kind;
mod queue_family_info;

mod enumerate_all_extensions;
mod enumerate_extensions;
mod eq;
mod get;
mod new;
mod ord;

pub use kind::VulkanAdapterKind;
pub use queue_family_info::VulkanQueueFamilyInfo;

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
    queue_families: Vec<VulkanQueueFamilyInfo>,

    /// The instance this adapter comes from
    instance: &'instance VulkanInstance,
}

unsafe impl<'instance> Send for VulkanAdapter<'instance> {}
unsafe impl<'instance> Sync for VulkanAdapter<'instance> {}
