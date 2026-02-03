use crate::git::git_repo_path;

/// Gets the current commit hash of the current working directory, if there is one
pub fn current_commit_hash() -> Option<String> {
    let git_repo = git_repo_path()?;

    let head = std::fs::read_to_string(git_repo.join("HEAD")).expect("cannot read git repo `HEAD`");
    let ref_path = match head.strip_prefix("ref: ") {
        Some(ref_path) => ref_path.trim(),
        None => return Some(head.trim().to_string()),
    };

    Some(
        std::fs::read_to_string(git_repo.join(ref_path))
            .expect("cannot read branch ref")
            .trim()
            .to_string(),
    )
}
