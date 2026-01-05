use crate::OsError;

impl std::error::Error for OsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OsError::Linux(error) => Some(error),
        }
    }
}
