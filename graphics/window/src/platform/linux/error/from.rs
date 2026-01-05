use crate::OsError;

impl From<linux::Error> for OsError {
    fn from(error: linux::Error) -> Self {
        OsError::Linux(error)
    }
}
