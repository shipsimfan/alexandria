use crate::CursorLock;

impl CursorLock {
    /// Get a string representing this cursor lock mode
    pub fn as_str(&self) -> &'static str {
        match self {
            CursorLock::Unlocked => "unlocked",
            CursorLock::Locked => "locked",
        }
    }
}
