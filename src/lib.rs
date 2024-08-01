//! A graphics library built on Vulkan

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod device;
mod instance;
mod window;

pub use device::{Device, DeviceCreateError};
pub use instance::{EnumeratePhysicalDevicesError, Instance, InstanceCreateError, PhysicalDevice};
pub use util::{Error, Severity};
pub use vk::{
    get_time_zone, message_box, EventCallback, VkPhysicalDeviceType as PhysicalDeviceType,
    WindowCreationError,
};
pub use window::Window;
