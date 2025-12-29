use crate::GraphicsLayer;

impl PartialEq for GraphicsLayer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.spec_version == other.spec_version
    }
}

impl Eq for GraphicsLayer {}
