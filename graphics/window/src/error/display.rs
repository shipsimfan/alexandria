use crate::WindowError;

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)?;

        if let Some(os) = &self.os {
            write!(f, " - {}", os)?;
        }

        Ok(())
    }
}
