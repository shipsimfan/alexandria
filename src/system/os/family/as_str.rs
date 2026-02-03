use crate::system::OsFamily;

impl OsFamily {
    /// Get a string representing this OS family
    pub const fn as_str(&self) -> &'static str {
        match self {
            OsFamily::Linux => "Linux",
            OsFamily::Windows => "Windows",
        }
    }
}
