use crate::program::{types::TypeManager, Type};
use std::rc::Rc;

impl TypeManager {
    /// Get an iterator over the contained types
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Rc<Type>> {
        self.types.iter()
    }
}

impl<'a> IntoIterator for &'a TypeManager {
    type Item = &'a Rc<Type>;
    type IntoIter = std::slice::Iter<'a, Rc<Type>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
