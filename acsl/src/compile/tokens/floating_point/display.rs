use crate::compile::tokens::FloatingPoint;

impl std::fmt::Display for FloatingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "float {}", self.value)
    }
}
