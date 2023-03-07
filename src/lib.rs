mod adapter;
mod display;
mod instance;
mod message_box;

pub use adapter::*;
pub use display::*;
pub use instance::*;
pub use message_box::*;

pub use common::MessageBoxClass;

#[cfg(target_os = "windows")]
pub type Error = dx12::Error;

pub type Result<T> = core::result::Result<T, Error>;
