use crate::ErrorInner;

impl ErrorInner {
    /// Get this value as a [`std::error::Error`]
    pub fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorInner::Other(_) => None,
        }
    }
}
