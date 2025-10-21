use crate::program::types::Vector;

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector {} #{}", self.name(), self.id())
    }
}
