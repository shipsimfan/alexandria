use crate::git::git_repo_path;

/// Gets the current branch of the current working directory, if there is one
pub fn current_branch() -> Option<String> {
    let git_repo = git_repo_path()?;

    let head = std::fs::read_to_string(git_repo.join("HEAD")).expect("cannot read git repo `HEAD`");
    head.strip_prefix("ref: refs/heads/")
        .map(|branch| branch.trim().to_owned())
}
