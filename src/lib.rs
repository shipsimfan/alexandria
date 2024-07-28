//! A graphics library built on Vulkan

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod instance;

pub use instance::{Instance, InstanceCreateError};
pub use util::{Error, Severity};
pub use vk::{message_box, Window, WindowCreationError};
