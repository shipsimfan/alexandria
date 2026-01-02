use crate::WindowError;

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.os.as_ref().map(|os| os as _)
    }
}
