//! Graphics library wrapping low-level graphics APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod math;

mod adapter;
mod error;
mod window;

pub use adapter::{Adapter, Output, OutputResolution};
pub use error::{Error, Result};
pub use window::Window;

const FORMAT: win32::dxgi::DXGI_FORMAT = win32::dxgi::DXGI_FORMAT::B8G8R8A8UNorm;
