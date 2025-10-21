use crate::program::types::Primitive;

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Primitive {} #{}", self.name(), self.id())
    }
}
