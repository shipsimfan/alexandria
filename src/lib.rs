mod instance;

pub use instance::Instance;

pub use vulkan::{Result, VkResult as Error};

#[cfg(target_os = "windows")]
pub(self) use windows as os;
