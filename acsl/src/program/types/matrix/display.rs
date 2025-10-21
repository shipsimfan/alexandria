use crate::program::types::Matrix;

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix {} #{}", self.name(), self.id())
    }
}
