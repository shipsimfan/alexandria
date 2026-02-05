//! Platform wrapper for various game APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
//
// Constant trait features
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_index)]
#![feature(const_ops)]
#![feature(const_option_ops)]
#![feature(const_slice_make_iter)]
#![feature(const_trait_impl)]
#![feature(const_try)]
//
// Other features
#![feature(associated_type_defaults)]
#![feature(formatting_options)]

mod context;
mod error;
mod misc;
mod notify;
mod shared_object;

pub mod gpu;
pub mod math;
pub mod system;
pub mod window;

#[cfg(feature = "git")]
pub mod git;

pub use context::*;
pub use error::*;
pub use misc::*;
pub use notify::*;
pub use shared_object::*;
