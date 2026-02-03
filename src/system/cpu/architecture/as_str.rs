use crate::system::Architecture;

impl Architecture {
    /// Get a string representing this architecture
    pub const fn as_str(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86-64",
        }
    }
}
