use crate::compile::tokens::FloatingPoint;

impl PartialEq for FloatingPoint {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
