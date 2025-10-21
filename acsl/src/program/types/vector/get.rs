use crate::program::{types::Vector, Type};
use std::rc::Rc;

impl Vector {
    /// Gets the name of this vector type
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the id of this vector type
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Gets the number of elements in vectors of this type
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Get the type of the elements
    pub fn r#type(&self) -> &Rc<Type> {
        &self.r#type
    }
}
