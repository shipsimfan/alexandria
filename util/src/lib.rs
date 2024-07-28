//! Common utilities for all crates in this workspace

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod cstr;
mod error;
mod flags;
mod severity;

pub use cstr::{i8_slice_to_cstr, slice_to_cstr};
pub use error::Error;
pub use severity::Severity;
