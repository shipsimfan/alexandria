mod adapter;
mod display;
mod instance;
mod message_box;

pub use adapter::*;
pub use display::*;
pub use instance::*;
pub use message_box::*;

pub type Error = win32::Win32Error;
pub type Result<T> = core::result::Result<T, Error>;

const DISPLAY_FORMAT: win32::DXGIFormat = win32::DXGIFormat::R8G8B8A8Unorm;
