use crate::GraphicsLayer;

impl std::fmt::Display for GraphicsLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.name, self.spec_version)
    }
}
