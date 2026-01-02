//! Window system interaction layer

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod builder;
mod error;
mod platform;
mod state;
mod wake_handle;

pub use builder::*;
pub use error::*;
pub use platform::*;
pub use state::*;
pub use wake_handle::*;
