//! Color related items

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
#![feature(const_ops)]
#![feature(const_trait_impl)]
//
// Other features
#![feature(formatting_options)]

mod color3;
mod color4;
mod color_space;

pub use color_space::*;
pub use color3::*;
pub use color4::*;
