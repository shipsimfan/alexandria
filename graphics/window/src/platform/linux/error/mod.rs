mod display;
mod error;
mod from;

/// The error that can come from the window system
#[derive(Debug)]
pub(crate) enum OsError {
    Linux(linux::Error),
}
