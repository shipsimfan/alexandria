//! Wrappers for Vulkan

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(negative_impls)]

mod create_error;
mod device;
mod functions;
mod global;
mod instance;

pub use create_error::CreateError;
pub use device::{Device, DeviceExtension, Queue, QueueCreateInfo};
pub use global::Global;
pub use instance::{
    DebugUtilsMessenger, EventCallback, Instance, InstanceExtension, InstanceLayer, PhysicalDevice,
    PhysicalDeviceProperties, Surface,
};
pub use os::{
    get_time_zone, instance_extensions as os_instance_extensions, message_box, Window,
    WindowCreationError,
};

pub use vulkan::{
    VkDeviceQueueCreateInfo, VkPhysicalDeviceType, VkPresentModeKHR, VkQueueFamilyProperties,
    VkQueueFlagBits, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows as os;
