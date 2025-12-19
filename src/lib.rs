//! Platform wrapper for various game APIs

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
//
// Constant trait features
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_clone)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_default)]

pub mod graphics;
pub mod math;
