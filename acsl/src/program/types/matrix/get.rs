use crate::program::{types::Matrix, Type};
use std::rc::Rc;

impl Matrix {
    /// Gets the name of this matrix type
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the id of this matrix type
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Gets the number of columns in matrices of this type
    pub fn columns(&self) -> u8 {
        self.columns
    }

    /// Gets the number of rows in matrices of this type
    pub fn rows(&self) -> u8 {
        self.rows
    }

    /// Get the type of the elements
    pub fn r#type(&self) -> &Rc<Type> {
        &self.r#type
    }
}
