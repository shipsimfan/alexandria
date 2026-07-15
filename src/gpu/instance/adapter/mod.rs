use crate::gpu::VulkanInstance;
use vulkan::VkPhysicalDevice;

mod features;
mod functions;
mod memory_properties;
mod properties;
mod queue_family_properties;
mod surface_capabilities;

mod device_builder;
mod enumerate_all_extensions;
mod enumerate_extensions;
mod get;
mod get_features;
mod get_memory_properties;
mod get_properties;
mod get_queue_family_properties;
mod new;
mod supports_surface;
mod surface_present_modes;
mod swapchain_formats;

pub use features::*;
pub use memory_properties::*;
pub use properties::*;
pub use queue_family_properties::*;
pub use surface_capabilities::*;

pub(in crate::gpu::instance) use functions::VulkanAdapterFunctions;

/// A physical device which can be used for Vulkan
pub struct VulkanAdapter<'instance> {
    /// The underlying Vulkan physical device
    handle: VkPhysicalDevice,

    /// The instance this adapter comes from
    instance: &'instance VulkanInstance,
}

unsafe impl<'instance> Send for VulkanAdapter<'instance> {}
unsafe impl<'instance> Sync for VulkanAdapter<'instance> {}
