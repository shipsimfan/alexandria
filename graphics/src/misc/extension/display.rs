use crate::GraphicsExtension;

impl std::fmt::Display for GraphicsExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.name, self.version)
    }
}
