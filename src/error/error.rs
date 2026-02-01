use crate::{Error, ErrorInner};

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.as_ref().and_then(ErrorInner::as_error)
    }
}
