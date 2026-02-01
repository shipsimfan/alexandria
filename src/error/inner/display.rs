use crate::ErrorInner;

impl std::fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorInner::Other(error) => error.fmt(f),
        }
    }
}
