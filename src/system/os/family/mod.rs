mod as_str;
mod display;

/// An OS family this may be running on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsFamily {
    /// The running operating system is Linux
    Linux,

    /// The running operating system is Windows
    Windows,
}
