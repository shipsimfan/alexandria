//! Utilities for getting information about the current git repository

use repo_path::git_repo_path;

mod current_branch;
mod current_commit;
mod repo_path;

pub use current_branch::current_branch;
pub use current_commit::current_commit_hash;
