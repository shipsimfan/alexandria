use crate::program::Type;

impl Type {
    /// Is this type a built-in type?
    pub fn is_builtin(&self) -> bool {
        match self {
            Type::Primitive(_) | Type::Vector(_) | Type::Matrix(_) => true,
        }
    }
}
