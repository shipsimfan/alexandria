//! Graphics focused math utilities

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
#![feature(const_trait_impl)]
#![feature(const_try)]
//
// Other features
#![feature(associated_type_defaults)]
#![feature(formatting_options)]

mod vector3;
mod vector4;

pub mod number;

pub use vector3::*;
pub use vector4::*;
