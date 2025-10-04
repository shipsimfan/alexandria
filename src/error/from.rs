use crate::Error;

impl From<win32::Error> for Error {
    fn from(error: win32::Error) -> Self {
        Error::System(error)
    }
}
