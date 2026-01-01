//! Utilities common to many alexandria projects

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_try)]

mod uuid;

pub use uuid::UUID;
