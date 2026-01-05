use crate::OsError;

impl std::fmt::Display for OsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OsError::Linux(error) => error.fmt(f),
        }
    }
}
