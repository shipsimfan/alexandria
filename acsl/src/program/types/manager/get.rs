use crate::program::{types::TypeManager, Type};
use std::rc::Rc;

impl TypeManager {
    /// Get the contained types a slice
    pub const fn as_slice(&self) -> &[Rc<Type>] {
        self.types.as_slice()
    }

    /// Get the number of types contained
    pub const fn len(&self) -> usize {
        self.types.len()
    }

    /// Get a [`Type`] by `name`
    pub fn get(&self, name: &str) -> Option<&Rc<Type>> {
        for r#type in &self.types {
            if r#type.name() == name {
                return Some(r#type);
            }
        }

        None
    }
}
