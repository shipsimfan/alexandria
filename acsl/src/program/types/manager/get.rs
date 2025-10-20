use crate::program::{types::TypeManager, Type};
use std::rc::Rc;

impl TypeManager {
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
