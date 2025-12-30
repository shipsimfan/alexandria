use crate::GraphicsAdapterKind;

impl std::fmt::Display for GraphicsAdapterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
