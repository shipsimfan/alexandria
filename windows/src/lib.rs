//! Wrappers for Windows

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod message_box;
mod window;

pub use message_box::message_box;
pub use window::{Window, WindowCreationError};
