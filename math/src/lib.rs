//! Graphics focused math utilities

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
//
// Constant trait features
#![feature(const_cmp)]
#![feature(const_trait_impl)]

mod vector3;

pub mod number;

pub use vector3::*;
