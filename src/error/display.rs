use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoValidAdapter => "there is no valid graphics adapter on this system".fmt(f),
            Error::System(error) => error.fmt(f),
        }
    }
}
