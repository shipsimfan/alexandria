//! Graphics API wrappers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(cstr_display)]
#![feature(box_as_ptr)]

mod device;
mod instance;
mod misc;
mod util;

pub use device::*;
pub use instance::*;
pub use misc::*;

pub use alexandria_color as color;
pub use alexandria_window as window;
