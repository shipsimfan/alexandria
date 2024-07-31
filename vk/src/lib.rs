//! Wrappers for Vulkan

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod create_error;
mod device;
mod functions;
mod global;
mod instance;

#[cfg(target_os = "windows")]
use windows as os;

pub use create_error::CreateError;
pub use device::Device;
pub use global::Global;
pub use instance::{
    DebugUtilsMessenger, EventCallback, Instance, InstanceExtension, InstanceLayer, PhysicalDevice,
    PhysicalDeviceProperties,
};
pub use os::{message_box, Window, WindowCreationError};

pub use vulkan::{
    VkDeviceQueueCreateInfo, VkPhysicalDeviceType, VkQueueFamilyProperties, VkQueueFlagBits,
    VkResult,
};
