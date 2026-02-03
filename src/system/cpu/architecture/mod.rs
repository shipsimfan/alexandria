mod as_str;
mod display;

/// An architecture the system may be
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    /// x86-64 architecture
    X86_64,
}
