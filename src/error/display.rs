use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)?;
        if let Some(inner) = &self.inner {
            " - ".fmt(f)?;
            inner.fmt(f)?;
        }
        Ok(())
    }
}
