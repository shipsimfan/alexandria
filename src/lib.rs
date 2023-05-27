mod device;
mod error;
mod graphics_context;
mod instance;
mod window;

pub use device::{Device, DEVICE_UUID_SIZE};
pub use error::{Error, ErrorKind, ErrorSource, Result};
pub use graphics_context::GraphicsContext;
pub use instance::Instance;
pub use vulkan::Version;
pub use window::Window;

pub(self) use error::create_error;

#[cfg(target_os = "windows")]
pub(self) use windows as os;
