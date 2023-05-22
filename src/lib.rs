mod error;
mod instance;
mod window;

pub use error::{Error, ErrorKind, ErrorSource, Result};
pub use instance::Instance;
pub use window::Window;

pub(self) use error::create_error;

#[cfg(target_os = "windows")]
pub(self) use windows as os;
