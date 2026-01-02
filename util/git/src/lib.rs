//! Utilities for getting information about the current git repository

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use repo_path::git_repo_path;

mod current_branch;
mod current_commit;
mod repo_path;

pub use current_branch::current_branch;
pub use current_commit::current_commit_hash;
