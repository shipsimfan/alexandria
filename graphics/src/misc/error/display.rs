use crate::GraphicsError;

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)?;

        if let Some(vk) = &self.vk {
            write!(f, " - {}", vk)?;
        }

        Ok(())
    }
}
