//! Graphics library wrapping low-level graphics APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod adapter;
mod error;
mod window;

pub use adapter::Adapter;
pub use error::{Error, Result};
pub use window::Window;
