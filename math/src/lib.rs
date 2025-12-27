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

mod matrix3x3;
mod matrix4x4;
mod quaternion;
mod vector2;
mod vector3;
mod vector4;

pub mod number;

pub use matrix3x3::*;
pub use matrix4x4::*;
pub use quaternion::*;
pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
