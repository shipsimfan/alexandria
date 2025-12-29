use crate::GraphicsVersion;

impl std::fmt::Display for GraphicsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant = self.variant();
        if variant != 0 {
            write!(f, "{}.", variant)?;
        }

        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}
