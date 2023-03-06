mod adapter;
mod instance;

pub use adapter::*;
pub use instance::*;

#[cfg(target_os = "windows")]
pub type Error = dx12::Error;

pub type Result<T> = core::result::Result<T, Error>;
