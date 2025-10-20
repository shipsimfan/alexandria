use crate::program::Type;
use std::rc::Rc;

mod add;
mod get;
mod new;

/// The list of all types in a program
#[derive(Debug)]
pub struct TypeManager {
    /// The list of types
    types: Vec<Rc<Type>>,
}
