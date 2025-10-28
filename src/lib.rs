//! Graphics library wrapping low-level graphics APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod graphics;
pub mod math;

mod error;
mod log_callbacks;
mod window;

pub use error::{Error, Result};
pub use log_callbacks::{GraphicsApiLogSeverity, LogCallbacks, StdoutLogger};
pub use window::{DisplayMode, Window, WindowBuilder};

pub use acsl;

const FORMAT: win32::dxgi::DXGI_FORMAT = win32::dxgi::DXGI_FORMAT::B8G8R8A8UNorm;
const BUFFER_COUNT: win32::UINT = 3;
