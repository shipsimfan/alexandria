//! Utilities common to many alexandria projects

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
#![feature(const_trait_impl)]
#![feature(const_try)]

mod memory_size;
mod uuid;

pub use memory_size::MemorySize;
pub use uuid::UUID;
