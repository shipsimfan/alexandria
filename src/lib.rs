//! Platform wrapper for various game APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_try)]

mod context;
mod error;
mod events;
mod misc;
mod notify;
mod shared_object;

pub mod gpu;
pub mod input;
pub mod system;
pub mod window;

#[cfg(feature = "git")]
pub mod git;

pub use time;

pub use context::*;
pub use error::*;
pub use events::*;
pub use misc::*;
pub use notify::*;
pub use shared_object::*;

pub use dioptra as math;
